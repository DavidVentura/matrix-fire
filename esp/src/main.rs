use esp_idf_hal::peripherals::Peripherals;
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
    let _w = wifi::configure(SSID, PASS, modem).expect("Could not configure wifi");
    println!("Wifi configured");
    let _h = http::server().expect("Could not start http server");
    println!("HTTP server up");

    let mut m = neopixels::Matrix::new(channel, led_pin);
    println!("Start matrix");

    loop {
        m.tick();
        sleep(Duration::from_millis(100));
    }
}
