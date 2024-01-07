use std::error::Error;
use std::thread;
use std::time::Duration;

use axum::{routing::get, Router};
// use rppal::gpio::Gpio;
// use rppal::system::DeviceInfo;

// Gpio uses BCM pin numbering. BCM GPIO 23 is tied to physical pin 16.
// const GPIO_LED: u8 = 23;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // build our application with a single route
    let app = Router::new().route("/", get(|| async { "Hello, Grow!" }));

    // run it with hyper on localhost:3000
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();

    // println!("Blinking an LED on a {}.", DeviceInfo::new()?.model());

    // let mut pin = Gpio::new()?.get(GPIO_LED)?.into_output();

    // // Blink the LED by setting the pin's logic level high for 500 ms.
    // pin.set_high();
    // thread::sleep(Duration::from_millis(500));
    // pin.set_low();

    Ok(())
}
