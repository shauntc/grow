//! Raspberry Pi Rust driver for the DHT22 temperature and humidity sensor, compatible with the [rppal](https://docs.golemparts.com/rppal/0.13.1/rppal/gpio/struct.IoPin.html#) GPIO library `IoPin` type.
//!
#![allow(unused)]
#![deny(unsafe_code)]
#![deny(missing_docs)]
#![cfg_attr(not(test), no_std)]

use embedded_hal::blocking::delay::{DelayMs, DelayUs};
use rppal::gpio::{IoPin, Mode};
use serde::{Deserialize, Serialize};

use super::{
    device::{Device, Measurement},
    Error, Result,
};

/// How long to wait for a pulse on the data line (in microseconds)
const TIMEOUT_US: u16 = 1_000;

/// How long to wait between successive retries (in milliseconds)
const RETRY_DELAY: u16 = 100;

/// A DHT22 device.
pub struct Dht22 {
    /// The concrete GPIO pin implementation.
    gpio: IoPin,
}

impl Dht22 {
    /// Creates a new DHT22 device connected to the specified pin.
    pub fn new(gpio: IoPin) -> Self {
        Dht22 { gpio }
    }

    /// Destroys the driver, returning the IoPin instance.
    pub fn destroy(self) -> IoPin {
        self.gpio
    }

    /// Attempts readings of the sensor up to `retries` times
    /// and returns the first successful reading or the last error
    pub fn perform_measurement_with_retries<D>(
        &mut self,
        delay: &mut D,
        retries: u16,
    ) -> Result<Measurement>
    where
        D: DelayUs<u16> + DelayMs<u16>,
    {
        let mut result = self.perform_measurement(delay);
        for _ in 0..retries {
            if result.is_ok() {
                break;
            }
            delay.delay_ms(RETRY_DELAY);
            result = self.perform_measurement(delay);
        }
        result
    }

    /// Performs a reading of the sensor.
    pub fn perform_measurement<D>(&mut self, delay: &mut D) -> Result<Measurement>
    where
        D: DelayUs<u16> + DelayMs<u16>,
    {
        let mut data = [0u8; 5];

        // Perform initial handshake
        self.perform_handshake(delay)?;

        // Read bits
        for i in 0..40 {
            data[i / 8] <<= 1;
            if self.read_bit(delay)? {
                data[i / 8] |= 1;
            }
        }

        // Finally wait for line to go idle again.
        //self.wait_for_pulse(true, delay)?;

        // Check CRC
        let crc = data[0]
            .wrapping_add(data[1])
            .wrapping_add(data[2])
            .wrapping_add(data[3]);
        if crc != data[4] {
            return Err(Error::CrcMismatch);
        }

        Ok(Self::raw_to_reading([data[0], data[1], data[2], data[3]]))
    }

    fn raw_to_reading(bytes: [u8; 4]) -> Measurement {
        let [rh_h, rh_l, temp_h_signed, temp_l] = bytes;
        let humidity = ((rh_h as u16) << 8 | (rh_l as u16)) as f32 / 10.0;
        let temperature = {
            let (signed, magnitude) = convert_signed(temp_h_signed);
            let temp_sign = if signed { -1.0 } else { 1.0 };
            let temp_magnitude = ((magnitude as u16) << 8) | temp_l as u16;
            temp_sign * temp_magnitude as f32 / 10.0
        };
        Measurement {
            temperature,
            humidity,
        }
    }

    fn perform_handshake<D>(&mut self, delay: &mut D) -> Result<()>
    where
        D: DelayUs<u16> + DelayMs<u16>,
    {
        self.gpio.set_mode(Mode::Output);
        // Set pin as floating to let pull-up raise the line and start the reading process.
        self.gpio.set_high();
        delay.delay_ms(1);

        // Pull line low for at least 18ms to send a start command.
        self.gpio.set_low();
        delay.delay_ms(20);

        // Restore floating
        self.gpio.set_high();
        delay.delay_us(40);

        self.gpio.set_mode(Mode::Input);

        // As a response, the device pulls the line low for 80us and then high for 80us.
        self.read_bit(delay)?;

        Ok(())
    }

    fn read_bit<D>(&mut self, delay: &mut D) -> Result<bool>
    where
        D: DelayUs<u16> + DelayMs<u16>,
    {
        let low = self.wait_for_pulse(true, delay)?;
        let high = self.wait_for_pulse(false, delay)?;
        Ok(high > low)
    }

    fn wait_for_pulse<D>(&mut self, level: bool, delay: &mut D) -> Result<u32>
    where
        D: DelayUs<u16> + DelayMs<u16>,
    {
        let mut count = 0;

        while self.gpio.is_high() != level {
            count += 1;
            if count > TIMEOUT_US {
                return Err(Error::Timeout);
            }
            delay.delay_us(1);
        }

        return Ok(u32::from(count));
    }
}

fn convert_signed(signed: u8) -> (bool, u8) {
    let sign = signed & 0x80 != 0;
    let magnitude = signed & 0x7F;
    (sign, magnitude)
}

impl Device for Dht22 {
    fn perform_measurement<D: DelayUs<u16> + DelayMs<u16>>(
        &mut self,
        delay: &mut D,
    ) -> Result<Measurement> {
        self.perform_measurement(delay)
    }

    fn perform_measurement_with_retries<D: DelayUs<u16> + DelayMs<u16>>(
        &mut self,
        delay: &mut D,
        retries: u16,
    ) -> Result<Measurement> {
        self.perform_measurement_with_retries(delay, retries)
    }
}
