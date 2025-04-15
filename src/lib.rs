pub mod requests;
pub mod ring;
pub mod responses;

use btleplug::api::{bleuuid::uuid_from_u16, Central, Manager as _, Peripheral as _, ScanFilter, WriteType};
use btleplug::platform::{Adapter, Manager, Peripheral};
use std::thread;
use std::time::Duration;
use std::error::Error;
use tokio::time;
use crate::ring::ColmiRing;
use zerocopy::{Immutable, KnownLayout, IntoBytes, FromBytes};

pub async fn find_rings() -> Result<Vec<ring::ColmiRing>, Box<dyn Error>> {

    let manager = Manager::new().await.unwrap();

    // get the first bluetooth adapter
    let adapters = manager.adapters().await?;
    let central = adapters.into_iter().nth(0).unwrap();

    // start scanning for devices
    central.start_scan(ScanFilter::default()).await?;
    time::sleep(Duration::from_secs(5)).await;

    let peripherals = central.peripherals().await?;
    let mut rings: Vec<ring::ColmiRing> = Vec::new();
    for p in peripherals {
        if let Ok(Some(props)) = p.properties().await {
            if let Some(name) = props.local_name {
                if name.starts_with("R0") {
                    rings.push(ColmiRing::new(p).await.unwrap());
                }
            }
        }
    }

    Ok(rings)
}

#[cfg(test)]
mod tests {
    use super::*;
    use zerocopy::FromBytes;

    #[test]
    fn it_works() {

        assert_eq!(2, 2);
    }
}
