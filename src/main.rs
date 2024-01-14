mod circular;
mod humidity;

use anyhow::Result;
use axum::{extract::State, http::StatusCode, routing::get, Router};
use circular::Circular;
use humidity::Update;
use rppal::gpio::Gpio;
use std::{env, future::IntoFuture, sync::Arc};

use tokio::{
    sync::RwLock,
    time::{interval, sleep, Duration},
};

type HumidityState = Arc<RwLock<Circular<humidity::Reading, 10>>>;

// Pins
const GPIO_HUMIDITY: u8 = 23;
// const GPIO_RELAY_1: u8 = 17;
// const GPIO_RELAY_2: u8 = 27;
// const GPIO_RELAY_3: u8 = 22;

impl Update for Circular<humidity::Reading, 10> {
    fn update(&mut self, reading: humidity::Reading) {
        self.add(reading);
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    println!("Running {}...", env::current_exe().unwrap().display());

    let humidity_tracker = humidity::Tracker::new(humidity::SensorType::Dht22, GPIO_HUMIDITY)?;
    let humidity_state: HumidityState = Arc::new(RwLock::new(Circular::new()));

    let update_task = humidity::start_tracking(
        humidity_state.clone(),
        humidity_tracker,
        interval(Duration::from_secs(2)),
    );

    // build our application with a single route
    let app = Router::new()
        .route("/", get(|| async { "Hello, Grow!" }))
        .route("/humidity", get(get_humidity))
        .route("/humidity/list", get(list_humidity))
        .route("/flash", get(flash_led))
        .with_state(humidity_state);

    // run it with hyper on localhost:3000
    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;

    let server = axum::serve(listener, app.into_make_service()).into_future();

    let _ = tokio::join!(server, update_task);

    Ok(())
}

async fn list_humidity(State(tracker): State<HumidityState>) -> String {
    let tracker = tracker.read().await;
    let mut result = String::new();
    for entry in tracker.iter() {
        result.push_str(&format!(
            "Temperature: {}°C, Humidity: {}%, Time: {}\n",
            entry.result.temperature, entry.result.humidity, entry.time
        ));
    }

    result
}

async fn get_humidity(State(tracker): State<HumidityState>) -> String {
    let tracker = tracker.read().await;

    match tracker.last() {
        Some(entry) => format!(
            "Temperature: {}°C, Humidity: {}%, Time: {}",
            entry.result.temperature, entry.result.humidity, entry.time
        ),
        None => "No data".to_owned(),
    }
}

async fn flash_led() -> StatusCode {
    tokio::spawn(async move {
        let _ = blink_led(5).await;
    });
    StatusCode::OK
}

const GPIO_LED: u8 = 23;
async fn blink_led(times: usize) -> Result<()> {
    let gpio = Gpio::new()?;
    let mut pin = gpio.get(GPIO_LED)?.into_output();

    for _ in 0..times {
        pin.set_high();
        sleep(Duration::from_millis(500)).await;
        pin.set_low();
        sleep(Duration::from_millis(500)).await;
    }

    Ok(())
}
