use std::cell::Cell;
use btleplug::api::{AddressType, Characteristic, Peripheral as _, PeripheralProperties, ValueNotification, WriteType};
use btleplug::platform::Peripheral;
use std::error::Error;
use std::ops::Deref;
use std::pin::Pin;
use std::sync::atomic::{AtomicBool, Ordering};
use std::task::Poll;
use tokio::sync::OnceCell;
use uuid::uuid;
use futures_core::Stream;
use tokio_stream::StreamExt;
use zerocopy::{FromBytes, Immutable, IntoBytes, KnownLayout};

pub struct ColmiRing {
    peripheral: Peripheral,
    props: OnceCell<PeripheralProperties>,
    req_char: Characteristic,
    res_char: Characteristic,
    notifications: Pin<Box<dyn Stream<Item = ValueNotification> + Send>>
}

impl ColmiRing {
    pub async fn new(peripheral: Peripheral) -> Result<Self, Box<dyn Error>> {

        if !peripheral.is_connected().await? {
            peripheral.connect().await?;
        }

        peripheral.discover_services().await?;

        let chars = peripheral.characteristics();
        let req_char = chars.iter().find(|c| c.uuid == uuid!("6e400002-b5a3-f393-e0a9-e50e24dcca9e")).unwrap().clone();
        let res_char = chars.iter().find(|c| c.uuid == uuid!("6e400003-b5a3-f393-e0a9-e50e24dcca9e")).unwrap().clone();

        peripheral.subscribe(&res_char).await?;
        let notifications = peripheral.notifications().await?;

        if let Ok(props) = peripheral.properties().await {
            Ok(ColmiRing { peripheral, props: OnceCell::new_with(props), req_char, res_char, notifications })
        }
        else {
            Ok(ColmiRing { peripheral, props: OnceCell::new(), req_char, res_char, notifications })
        }
    }

    pub fn print_info(&self) {
        if let Some(props) = self.props.get() {
            if let Some(name) = &props.local_name {
                println!("Name: {}", name);
            }
            println!("Address: {}", props.address);
        }

        let chars = self.peripheral.characteristics();
        for char in chars {
            println!("CHAR: {:?}", char);
            println!("  SERVICE UUID: {:?}", char.service_uuid);
            println!("  CHAR UUID:    {:?}", char.uuid);
            println!("  PROPERTIES");
            for prop in char.properties {
                println!("    {:?}", prop);
            }
            println!("  DESCRIPTORS");
            for descriptor in char.descriptors {
                println!("    {:?}", descriptor);
            }
        }
    }

    pub async fn disconnect(&self) -> btleplug::Result<()> {
        self.peripheral.disconnect().await
    }

    pub async fn send_cmd<T>(&self, cmd: T) -> btleplug::Result<()>
        where T: IntoBytes + Immutable {
        let mut bytes = cmd.as_bytes().to_vec();
        let checksum = bytes.iter().fold(0u16, |b, x| { b + (*x as u16) }) & 0xFF;
        bytes.resize(15, 0);
        bytes.push(checksum as u8);
        self.peripheral.write(&self.req_char, &bytes, WriteType::WithoutResponse).await
    }

    pub async fn send_cmd_with_response<T, U>(&self, cmd: T) -> btleplug::Result<Option<U>>
    where T: IntoBytes + Immutable, U: FromBytes + KnownLayout + Immutable {
        let mut bytes = cmd.as_bytes().to_vec();
        let checksum = bytes.iter().fold(0u16, |b, x| { b + (*x as u16) }) & 0xFF;
        bytes.resize(15, 0);
        bytes.push(checksum as u8);
        println!("{:?}", self.req_char);
        println!("{:?}", bytes);
        self.peripheral.write(&self.req_char, &bytes, WriteType::WithoutResponse).await?;
        println!("Request sent");
        let mut notifications = self.peripheral.notifications().await?;
        loop {
            let next = notifications.next().await;
            if let Some(v) = next {
                println!("Notification: {:?}", v);
                if v.value[0] == bytes[0] {
                    let data = U::read_from_bytes(&v.value[0..v.value.len()]).unwrap();
                    return btleplug::Result::Ok(Some(data));
                }
            }
        }
    }
}