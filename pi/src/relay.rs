use anyhow::Result;
use rppal::gpio::{Gpio, OutputPin};

#[derive(Debug)]
pub struct Relay {
    pin: OutputPin,
    pub on: bool,
}

impl Relay {
    pub fn new(gpio_pin: u8) -> Result<Self> {
        let gpio = Gpio::new()?;
        let pin = gpio.get(gpio_pin)?.into_output();
        let on = pin.is_set_high();
        Ok(Relay { pin, on })
    }

    pub fn on(&mut self) {
        self.pin.set_high();
        self.on = self.pin.is_set_high();
    }

    pub fn off(&mut self) {
        self.pin.set_low();
        self.on = self.pin.is_set_high();
    }
}
