use esp_idf_hal::peripherals::Peripherals;
use esp_idf_sys::*;
use num_traits::NumCast;
use render::Fire;
//use smart_leds::hsv::{hsv2rgb, Hsv};
use smart_leds::RGB8;
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

    let mut f = Fire::new(32, 8, 1, 1.6, rand, PALETTE);

    let pix: &mut [u8] = &mut [0; WIDTH as usize * HEIGHT as usize];
    let black = RGB8 { r: 0, g: 0, b: 0 };
    let mut i = 0;
    loop {
        i = (i + 1) % (WIDTH * 8);
        let mut pix_bottom: Vec<RGB8> = (0..(32 * HEIGHT)).map(|_| black).collect();
        _update_fire_intensity(pix, rand);
        /*
        for x in 0..WIDTH {
            for y in 0..8 {
                {
                    //for _ in 0..2 {
                    // 20
                    let idx = y * WIDTH + x;
                    let palete_idx = pix[idx as usize];
                    let color = PALETTE[palete_idx as usize];
                    let r = RGB8 {
                        r: 128 / (y + 1) as u8, //((color & 0xff0000) >> 16) as u8 >> 2,
                        g: y as u8 * 8,         //((color & 0x00ff00) >> 8) as u8 >> 2,
                        b: 0,                   //((color & 0x0000ff) >> 0) as u8 >> 2,
                    };
                    //pix_bottom.push(r);
                    if i < y * WIDTH + x {
                        pix_bottom.push(r); //[idx as usize] = r;
                                            //pix_bottom.push(r); //[idx as usize] = r;
                    } else {
                        pix_bottom.push(black);
                    }
                }
            }
        }
        */
        // these screens zig-zag:
        // | 00 01 02 03 04 |
        // | 09 08 07 06 05 |
        // | 10 11 12 13 14 |
        //for y in 0..8 {
        {
            for x in 0..32 {
                // 20
                let r = RGB8 {
                    r: 64 / (0 + 1) as u8,
                    g: x as u8 * 4,
                    b: 0,
                };
                pix_bottom[(0 * 32 + x) as usize] = r;
            }
        }
        //for y in 0..8 {
        //pix_bottom.clear();
        //pix_bottom.push(RGB8 { r: 128, g: 0, b: 0 });
        //pix_bottom.push(RGB8 { r: 0, g: 128, b: 0 });
        //pix_bottom.push(RGB8 { r: 0, g: 0, b: 128 });
        /*let pixels = std::iter::repeat(hsv2rgb(Hsv {
            hue,
            sat: 255,
            val: 1,
        }))
        .take(25);
        */
        ws2812.write(pix_bottom.into_iter()).unwrap();

        sleep(Duration::from_millis(100));
    }
}
