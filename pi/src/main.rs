mod circular;
mod humidity;
mod relay;
mod sensor_data;

use anyhow::Result;
use axum::{
    extract::{Path, State},
    http::{Method, StatusCode},
    routing::get,
    Json, Router,
};
use circular::Circular;
use humidity::Update;
use relay::{Relay, RelayBoard};
use rppal::gpio::Gpio;
use sensor_data::SensorData;
use std::{env, future::IntoFuture, sync::Arc};
use tokio::{
    sync::RwLock,
    time::{interval, sleep, Duration},
};
use tower_http::cors::{Any, CorsLayer};

type HumidityState = Arc<RwLock<Circular<humidity::Reading, 10>>>;
type RelayState = Arc<RwLock<RelayBoard<3>>>;

// Pins
const GPIO_HUMIDITY: u8 = 23;
const GPIO_RELAY_1: u8 = 17;
const GPIO_RELAY_2: u8 = 27;
const GPIO_RELAY_3: u8 = 22;

impl Update for Circular<humidity::Reading, 10> {
    fn update(&mut self, reading: humidity::Reading) {
        self.add(reading);
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    println!("Running {}...", env::current_exe().unwrap().display());

    // humidity sensor setup
    let humidity_tracker = humidity::Tracker::new(humidity::SensorType::Dht22, GPIO_HUMIDITY)?;
    let humidity_state: HumidityState = Arc::new(RwLock::new(Circular::new()));
    let update_task = humidity::start_tracking(
        humidity_state.clone(),
        humidity_tracker,
        interval(Duration::from_secs(2)),
    );

    let relays = Arc::new(RwLock::new(relay::RelayBoard::new([
        GPIO_RELAY_1,
        GPIO_RELAY_2,
        GPIO_RELAY_3,
    ])?));

    let cors = CorsLayer::new()
        // allow `GET` and `POST` when accessing the resource
        .allow_methods([Method::GET, Method::POST])
        // allow requests from any origin
        .allow_origin(Any);

    // build our application with a single route
    let app = Router::new()
        .route("/", get(|| async { "Hello, Grow!" }))
        .route("/sensors", get(get_sensor_data))
        .route("/humidity", get(get_humidity))
        .route("/humidity/list", get(list_humidity))
        .with_state(humidity_state)
        .route("/relay/:id/toggle", get(toggle_relay))
        .route("/relay/:id/on", get(relay_on))
        .route("/relay/:id/off", get(relay_off))
        .with_state(relays)
        .layer(cors);

    // run it with hyper on localhost:3000
    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;

    let server = axum::serve(listener, app.into_make_service()).into_future();

    let _ = tokio::join!(server, update_task);

    Ok(())
}

async fn get_sensor_data(State(tracker): State<HumidityState>) -> Json<Option<SensorData>> {
    let tracker = tracker.read().await;

    match tracker.last() {
        Some(entry) => Json(Some(SensorData::from(entry.result))),
        None => Json(None),
    }
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

async fn toggle_relay(Path(id): Path<usize>, State(relays): State<RelayState>) -> StatusCode {
    let mut relays = relays.write().await;
    let relay = match relays.get_mut(id) {
        Some(relay) => relay,
        None => return StatusCode::NOT_FOUND,
    };

    relay.toggle();

    StatusCode::OK
}
async fn relay_on(Path(id): Path<usize>, State(relays): State<RelayState>) -> StatusCode {
    let mut relays = relays.write().await;
    let relay = match relays.get_mut(id) {
        Some(relay) => relay,
        None => return StatusCode::NOT_FOUND,
    };

    relay.on();

    StatusCode::OK
}

async fn relay_off(Path(id): Path<usize>, State(relays): State<RelayState>) -> StatusCode {
    let mut relays = relays.write().await;
    let relay = match relays.get_mut(id) {
        Some(relay) => relay,
        None => return StatusCode::NOT_FOUND,
    };

    relay.off();

    StatusCode::OK
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
