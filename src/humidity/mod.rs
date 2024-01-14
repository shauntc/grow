mod device;
mod dht11;
mod dht22;
mod error;
mod tracker;

use std::sync::Arc;

use chrono::DateTime;
use serde::{Deserialize, Serialize};
use tokio::{sync::RwLock, task::JoinHandle, time::Interval};

use device::Measurement;
pub use error::{Error, Result};
pub use tracker::Tracker;

#[allow(dead_code)]
pub enum SensorType {
    Dht22,
    Dht11,
}

enum Sensor {
    Dht22(dht22::Dht22),
    Dht11(dht11::Dht11),
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Reading {
    pub result: Measurement,
    pub time: DateTime<chrono::Utc>,
}

pub trait Update {
    fn update(&mut self, reading: Reading);
    fn error(&mut self, error: Error) {
        println!("Error: {:?}", error);
    }
}

pub fn start_tracking<T: Update + Send + Sync + 'static>(
    state: Arc<RwLock<T>>,
    mut tracker: Tracker,
    mut interval: Interval,
) -> JoinHandle<()> {
    tokio::task::spawn(async move {
        loop {
            interval.tick().await;
            let read_result = tracker.read();
            match read_result {
                Ok(reading) => {
                    state.write().await.update(reading);
                }
                Err(e) => {
                    println!("Error reading sensor: {:?}", e);
                }
            }
        }
    })
}
