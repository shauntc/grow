use super::Result;
use embedded_hal::blocking::delay::{DelayMs, DelayUs};
use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Default, Debug, Serialize, Deserialize)]
pub struct Measurement {
    /// The measured temperature in tenths of degrees Celsius.
    pub temperature: f32,
    /// The measured humidity in tenths of a percent.
    pub humidity: f32,
}

pub trait Device {
    fn perform_measurement<D: DelayUs<u16> + DelayMs<u16>>(
        &mut self,
        delay: &mut D,
    ) -> Result<Measurement>;
    fn perform_measurement_with_retries<D: DelayUs<u16> + DelayMs<u16>>(
        &mut self,
        delay: &mut D,
        retries: u16,
    ) -> Result<Measurement>;
}
