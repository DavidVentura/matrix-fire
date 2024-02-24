use esp_idf_hal::gpio::OutputPin;
use esp_idf_hal::peripheral::Peripheral;
use esp_idf_hal::rmt::RmtChannel;
use esp_idf_sys::*;
use num_traits::NumCast;
use render::{pixmap, Fire, PALETTE};
use smart_leds::{brightness, gamma, RGB8};
use smart_leds_trait::SmartLedsWrite;
use ws2812_esp32_rmt_driver::Ws2812Esp32Rmt;

pub(crate) struct Matrix<'a> {
    ws2812: Ws2812Esp32Rmt<'a>,
    f: Fire<'a, RGB8>,
    pix: Vec<RGB8>,
    brightness: u8,
}

impl<'a> Matrix<'a> {
    pub(crate) fn new(
        channel: impl Peripheral<P = impl RmtChannel> + 'a,
        led_pin: impl Peripheral<P = impl OutputPin> + 'a,
    ) -> Matrix<'a> {
        let ws2812 = Ws2812Esp32Rmt::new(channel, led_pin).unwrap();

        let f = Fire::new(20, 16, 2.6, rand, &PALETTE);
        let black = RGB8 { r: 0, g: 0, b: 0 };
        let pix_bottom: Vec<RGB8> = (0..(1280)).map(|_| black).collect();

        Matrix {
            ws2812,
            f,
            pix: pix_bottom,
            brightness: 255,
        }
    }
    pub(crate) fn tick(&mut self) {
        self.f.update_fire_intensity();

        for p in self.f.into_iter() {
            // upscale to 2x2
            self.pix[pixmap(p.x as u16 * 2 + 0, p.y as u16 * 2 + 0) as usize] = p.c;
            self.pix[pixmap(p.x as u16 * 2 + 0, p.y as u16 * 2 + 1) as usize] = p.c;
            self.pix[pixmap(p.x as u16 * 2 + 1, p.y as u16 * 2 + 0) as usize] = p.c;
            self.pix[pixmap(p.x as u16 * 2 + 1, p.y as u16 * 2 + 1) as usize] = p.c;
        }
        let iter = (&self.pix).into_iter().copied();
        self.ws2812
            .write(brightness(gamma(iter), self.brightness))
            .unwrap();
    }
    pub(crate) fn set_brightness(&mut self, brightness: u8) {
        self.brightness = brightness;
    }
}
fn rand() -> f32 {
    let v: u32 = unsafe { esp_random() };
    <f32 as NumCast>::from(v).unwrap() / <f32 as NumCast>::from(u32::MAX).unwrap()
}
