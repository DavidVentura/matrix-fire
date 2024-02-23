use esp_idf_hal::peripherals::Peripherals;
use esp_idf_sys::*;
use num_traits::NumCast;
use render::{pixmap, Fire, PALETTE};
use smart_leds::{brightness, gamma, RGB8};
use smart_leds_trait::SmartLedsWrite;
use std::thread::sleep;
use std::time::Duration;
use ws2812_esp32_rmt_driver::Ws2812Esp32Rmt;

fn rand() -> f32 {
    let v: u32 = unsafe { esp_random() };
    <f32 as NumCast>::from(v).unwrap() / <f32 as NumCast>::from(u32::MAX).unwrap()
}

fn main() -> ! {
    // Temporary. Will disappear once ESP-IDF 4.4 is released, but for now it is necessary to call this function once,
    // or else some patches to the runtime implemented by esp-idf-sys might not link properly.
    esp_idf_sys::link_patches();

    let peripherals = Peripherals::take().unwrap();
    let led_pin = peripherals.pins.gpio27;
    let channel = peripherals.rmt.channel0;
    let mut ws2812 = Ws2812Esp32Rmt::new(channel, led_pin).unwrap();

    println!("Start NeoPixel rainbow!");

    let mut f = Fire::new(20, 16, 2.6, rand, &PALETTE, true);

    let black = RGB8 { r: 0, g: 0, b: 0 };
    let mut _i: u16 = 0;

    //let data: Vec<u16> = (0..20).collect();
    loop {
        f.update_fire_intensity();
        let mut pix_bottom: Vec<RGB8> = (0..(1280)).map(|_| black).collect();

        /*
        i = (i + 1) % (20);
        {
            let r = RGB8 {
                r: 64 as u8,
                g: (i * 4) as u8,
                b: 0,
            };
            //pix_bottom[pixmap(0, data[i as usize]) as usize] = r;
            pix_bottom[pixmap(data[i as usize] * 2, 0) as usize] = r;
        }
        */
        for p in f.into_iter() {
            pix_bottom[pixmap(p.x as u16 * 2 + 0, p.y as u16 * 2 + 0) as usize] = p.c;
            pix_bottom[pixmap(p.x as u16 * 2 + 0, p.y as u16 * 2 + 1) as usize] = p.c;
            pix_bottom[pixmap(p.x as u16 * 2 + 1, p.y as u16 * 2 + 0) as usize] = p.c;
            pix_bottom[pixmap(p.x as u16 * 2 + 1, p.y as u16 * 2 + 1) as usize] = p.c;
        }
        ws2812
            //.write(gamma(pix_bottom.into_iter()))
            .write(brightness(gamma(pix_bottom.into_iter()), 50))
            .unwrap();

        sleep(Duration::from_millis(100));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert!(upscale(0) == (0, 1, 14, 15));
        assert!(upscale(1) == (2, 3, 12, 13));
        assert!(upscale(3) == (6, 7, 8, 9));
        assert!(upscale(4) == (16, 17, 30, 31));
    }
}
