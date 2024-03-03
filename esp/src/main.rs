use esp_idf_hal::cpu::Core;
use esp_idf_hal::peripherals::Peripherals;
use esp_idf_hal::task::thread::ThreadSpawnConfiguration;
use std::sync::mpsc;
use std::thread::sleep;
use std::time::{Duration, Instant};

mod http;
mod neopixels;
mod wifi;

const SSID: &'static str = env!("SSID");
const PASS: &'static str = env!("PASS");

fn main() -> ! {
    esp_idf_sys::link_patches();

    esp_idf_logger::init().unwrap();
    let peripherals = Peripherals::take().unwrap();
    let modem = peripherals.modem;
    let led_pin = peripherals.pins.gpio27;
    let channel = peripherals.rmt.channel0;

    let (tx, rx) = mpsc::channel();
    ThreadSpawnConfiguration {
        pin_to_core: Some(Core::Core0),
        priority: 1,
        ..Default::default()
    }
    .set()
    .expect("Cannot set thread spawn config");

    std::thread::spawn(move || {
        let wifi_start = Instant::now();
        let _w = wifi::configure(SSID, PASS, modem).expect("Could not configure wifi");
        println!("Wifi configured, took {:?}", wifi_start.elapsed());

        let _h = http::server(tx).expect("Could not start http server");
        println!("HTTP server up");
        loop {
            sleep(Duration::from_millis(50));
        }
    });

    println!("Start matrix");
    ThreadSpawnConfiguration {
        pin_to_core: Some(Core::Core1),
        priority: 24,
        ..Default::default()
    }
    .set()
    .expect("Cannot set thread spawn config");
    std::thread::spawn(move || {
        // `Matrix` must be created in Core1, as the RMT driver
        // stores its current affinity upon **creation**;
        // the RMT driver will spawn a thread later, which disregards
        // the current ThreadSpawnConfiguration.
        let mut m = neopixels::Matrix::new(channel, led_pin);

        loop {
            m.tick();
            match rx.recv_timeout(Duration::from_millis(50)) {
                Err(_) => continue,
                Ok(http::Commands::Brightness(v)) => m.set_brightness(v),
            }
        }
    });

    loop {
        sleep(Duration::from_millis(100));
    }
}
