use std::cell::Cell;
use std::collections::HashMap;
use btleplug::api::{AddressType, Characteristic, Peripheral as _, PeripheralProperties, ValueNotification, WriteType};
use btleplug::platform::Peripheral;
use std::error::Error;
use std::ops::Deref;
use std::pin::Pin;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::Receiver;
use std::task::Poll;
use tokio::sync::OnceCell;
use tokio;
use tokio::sync::{watch, broadcast, oneshot};
use uuid::uuid;
use futures_core::Stream;
use tokio::task::{yield_now, JoinSet};
use tokio_stream::StreamExt;
use zerocopy::{FromBytes, Immutable, IntoBytes, KnownLayout};
use super::requests::*;

pub struct ColmiRing {
    peripheral: Peripheral,
    props: OnceCell<PeripheralProperties>,
    req_char: Characteristic,
    res_char: Characteristic,
    notifs: Pin<Box<dyn Stream<Item = ValueNotification> + Send>>,
    notifs_rx: broadcast::Receiver<[u8; 16]>,
    tasks: JoinSet<btleplug::Result<()>>,
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
        let mut notifs = peripheral.notifications().await?;
        let (tx, notifs_rx) = broadcast::channel(16);
        let mut tasks = JoinSet::new();
        tasks.spawn(async move {
            while let Some(notif) = notifs.next().await {
                let bytes = notif.value.as_slice().try_into().unwrap();
                //println!("{:?}", &bytes);
                tx.send(bytes).unwrap();
            }
            Ok(())
        });

        let notifs = peripheral.notifications().await?;

        if let Ok(props) = peripheral.properties().await {
            Ok(ColmiRing { peripheral, props: OnceCell::new_with(props), req_char, res_char, notifs, notifs_rx, tasks })
        }
        else {
            Ok(ColmiRing { peripheral, props: OnceCell::new(), req_char, res_char, notifs, notifs_rx, tasks })
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

    pub async fn disconnect(&mut self) -> btleplug::Result<()> {
        self.tasks.abort_all();
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

    pub async fn send_cmd_with_response<T, U>(&mut self, cmd: T) -> btleplug::Result<oneshot::Receiver<U>>
    where T: IntoBytes + Immutable, U: FromBytes + KnownLayout + Immutable + Send + 'static {
        let mut bytes = cmd.as_bytes().to_vec();
        let checksum = bytes.iter().fold(0u16, |b, x| { b + (*x as u16) }) & 0xFF;
        bytes.resize(15, 0);
        bytes.push(checksum as u8);
        self.peripheral.write(&self.req_char, &bytes, WriteType::WithoutResponse).await?;
        let (tx, mut rx) = oneshot::channel();
        let mut notifs_rx = self.notifs_rx.resubscribe();
        self.tasks.spawn(async move {
            while let Ok(notif) = notifs_rx.recv().await {
                if notif[0] == bytes[0] {
                    let (data, _) = U::read_from_prefix(&notif).unwrap();
                    match tx.send(data) {
                        Ok(_) => {}
                        Err(_) => {}
                    }
                    break;
                }
            }
            Ok(())
        });
        Ok(rx)
    }

    pub async fn manage_data_stream(&mut self, data_type: u8, data_action: DataAction) -> btleplug::Result<()> {
        let cmd = DataRequest::new(data_type, data_action);
        let mut bytes = cmd.as_bytes().to_vec();
        let checksum = bytes.iter().fold(0u16, |b, x| { b + (*x as u16) }) & 0xFF;
        bytes.resize(15, 0);
        bytes.push(checksum as u8);
        self.peripheral.write(&self.req_char, &bytes, WriteType::WithoutResponse).await?;
        Ok(())
    }

    pub fn subscribe(&self) -> broadcast::Receiver<[u8; 16]> {
        self.notifs_rx.resubscribe()
    }
}