use crate::humidity;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct SensorData {
    temperature: f32,
    humidity: f32,
}

impl SensorData {
    pub fn from(measurement: humidity::Measurement) -> Self {
        Self {
            temperature: measurement.temperature,
            humidity: measurement.humidity,
        }
    }
}
