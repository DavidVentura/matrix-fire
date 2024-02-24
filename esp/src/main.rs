use esp_idf_hal::peripherals::Peripherals;
use std::sync::{Arc, Mutex};
use std::thread::sleep;
use std::time::Duration;

mod http;
mod neopixels;
mod wifi;

const SSID: &'static str = env!("SSID");
const PASS: &'static str = env!("PASS");

fn main() -> ! {
    // Temporary. Will disappear once ESP-IDF 4.4 is released, but for now it is necessary to call this function once,
    // or else some patches to the runtime implemented by esp-idf-sys might not link properly.
    esp_idf_sys::link_patches();

    let peripherals = Peripherals::take().unwrap();
    let modem = peripherals.modem;
    let led_pin = peripherals.pins.gpio27;
    let channel = peripherals.rmt.channel0;
    let m = neopixels::Matrix::new(channel, led_pin);
    let guard1 = Arc::new(Mutex::new(m));
    let guard2 = guard1.clone();
    println!("Start matrix");

    std::thread::spawn(move || {
        let _w = wifi::configure(SSID, PASS, modem).expect("Could not configure wifi");
        println!("Wifi configured");

        let _h = http::server(move |val: u8| {
            guard2.lock().unwrap().set_brightness(val);
            println!("Set brightness to {}", val);
        })
        .expect("Could not start http server");
        println!("HTTP server up");
        loop {
            sleep(Duration::from_secs(1));
        }
    });

    println!("Ticking");
    loop {
        guard1.lock().unwrap().tick();
        sleep(Duration::from_millis(100));
    }
}
