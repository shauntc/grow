use std::array;

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

    pub fn toggle(&mut self) {
        if self.on {
            self.off();
        } else {
            self.on();
        }
    }
}

#[derive(Debug)]
pub struct RelayBoard<const N: usize> {
    relays: [Relay; N],
}

impl<const N: usize> RelayBoard<N> {
    pub fn new(pin_nums: [u8; N]) -> Result<Self> {
        let relays = pin_nums
            .into_iter()
            .map(Relay::new)
            .collect::<Result<Vec<Relay>>>()?;

        let relays = match relays.try_into() {
            Ok(relays) => relays,
            Err(_) => unreachable!("Relay count cannot mismatch because the function signature requires the specified count")
        };

        Ok(RelayBoard { relays })
    }

    pub fn get_mut(&mut self, index: usize) -> Option<&mut Relay> {
        self.relays.get_mut(index)
    }
}
