use super::{dht11::Dht11, dht22::Dht22, Reading, Result, Sensor, SensorType};

use rppal::{
    gpio::{Gpio, Mode},
    hal::Delay,
};

pub struct Tracker {
    sensor: Sensor,
}

impl Tracker {
    pub fn new(sensor_type: SensorType, gpio_pin: u8) -> Result<Self> {
        let gpio = Gpio::new()?;
        let pin = gpio.get(gpio_pin)?.into_io(Mode::Input);

        match sensor_type {
            SensorType::Dht22 => Ok(Tracker {
                sensor: Sensor::Dht22(Dht22::new(pin)),
            }),
            SensorType::Dht11 => Ok(Tracker {
                sensor: Sensor::Dht11(Dht11::new(pin)),
            }),
        }
    }

    pub fn read(&mut self) -> Result<Reading> {
        match self.sensor {
            Sensor::Dht22(ref mut dht22) => {
                match dht22.perform_measurement_with_retries(&mut Delay, 10) {
                    Ok(result) => Ok(Reading {
                        result,
                        time: chrono::Utc::now(),
                    }),
                    Err(e) => Err(e),
                }
            }
            Sensor::Dht11(ref mut dht11) => {
                match dht11.perform_measurement_with_retries(&mut Delay, 10) {
                    Ok(result) => Ok(Reading {
                        result,
                        time: chrono::Utc::now(),
                    }),
                    Err(e) => Err(e),
                }
            }
        }
    }
}
