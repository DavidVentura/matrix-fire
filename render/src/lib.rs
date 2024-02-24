use rgb::RGB8;
use smart_leds::hsv::Hsv;
pub static PALETTE_HSV: [Hsv; 11] = [
    Hsv {
        hue: 00,
        sat: 00,
        val: 000,
    },
    Hsv {
        hue: 25,
        sat: 67,
        val: 019,
    },
    Hsv {
        hue: 26,
        sat: 72,
        val: 034,
    },
    Hsv {
        hue: 24,
        sat: 75,
        val: 046,
    },
    Hsv {
        hue: 21,
        sat: 99,
        val: 082,
    },
    Hsv {
        hue: 39,
        sat: 88,
        val: 087,
    },
    Hsv {
        hue: 34,
        sat: 91,
        val: 100,
    },
    Hsv {
        hue: 41,
        sat: 91,
        val: 099,
    },
    Hsv {
        hue: 47,
        sat: 90,
        val: 100,
    },
    Hsv {
        hue: 56,
        sat: 91,
        val: 100,
    },
    Hsv {
        hue: 62,
        sat: 71,
        val: 100,
    },
];

pub static PALETTE: [RGB8; 11] = [
    RGB8 {
        r: 0x00,
        g: 0x00,
        b: 0x00,
    },
    RGB8 {
        r: 0x31,
        g: 0x1E,
        b: 0x10,
    },
    RGB8 {
        r: 0x57,
        g: 0x33,
        b: 0x18,
    },
    RGB8 {
        r: 0x76,
        g: 0x41,
        b: 0x1E,
    },
    RGB8 {
        r: 0xD1,
        g: 0x4B,
        b: 0x02,
    },
    RGB8 {
        r: 0xDE,
        g: 0x9A,
        b: 0x1A,
    },
    RGB8 {
        r: 0xFF,
        g: 0x9C,
        b: 0x17,
    },
    RGB8 {
        r: 0xFE,
        g: 0xB4,
        b: 0x17,
    },
    RGB8 {
        r: 0xFF,
        g: 0xCF,
        b: 0x19,
    },
    RGB8 {
        r: 0xFE,
        g: 0xED,
        b: 0x18,
    },
    RGB8 {
        r: 0xFA,
        g: 0xFF,
        b: 0x4A,
    },
];

pub struct Fire<'a, T: Copy> {
    pub(crate) width: u8,
    pub(crate) height: u8,
    decay_factor: f32,
    rand_fn: fn() -> f32,
    pub(crate) palette: &'a [T],
    pub(crate) intensity_data: Vec<Vec<u8>>,
}

pub struct FireIterator<'a, T: Copy> {
    f: &'a Fire<'a, T>,
    curr_x: usize,
    curr_y: usize,
}

#[derive(Debug)]
pub struct ColorCoord<T: Copy> {
    pub c: T,
    pub x: u8,
    pub y: u8,
}

/*
impl<T, A> Into<A> for ColorCoord<T> {
    fn into(self) -> A {
        self.c
    }
}
*/

impl<'a, T: Copy> Iterator for FireIterator<'a, T> {
    type Item = ColorCoord<T>;

    fn next(&mut self) -> Option<Self::Item> {
        let ret = if self.curr_y == (self.f.height as usize) {
            None
        } else {
            let firstbit = &self.f.intensity_data[self.curr_x];
            let c = self.f.palette[firstbit[self.curr_y] as usize];
            Some(ColorCoord {
                c,
                x: self.curr_x as u8,
                y: self.curr_y as u8,
            })
        };

        self.curr_x += 1;
        if self.curr_x == self.f.width as usize {
            self.curr_y += 1;
            self.curr_x = 0;
        }
        ret
    }
}

impl<'a, T: Copy> IntoIterator for &'a Fire<'a, T> {
    type Item = ColorCoord<T>;
    type IntoIter = FireIterator<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        FireIterator {
            f: &self,
            curr_x: 0,
            curr_y: 0,
        }
    }
}
impl<'a, T: Copy> Fire<'a, T> {
    pub fn new(
        width: u8,
        height: u8,
        decay_factor: f32,
        rand_fn: fn() -> f32,
        palette: &'a [T],
    ) -> Fire<'a, T> {
        Fire {
            width,
            height,
            decay_factor,
            rand_fn,
            palette,
            intensity_data: vec![vec![0; height as usize]; width as usize],
        }
    }

    pub fn set_palette(&mut self, palette: &'a [T]) {
        self.palette = palette;
    }
    fn update_fire_pixel_intensity(&mut self, x: u8, y: u8) {
        let decay = ((self.rand_fn)() * self.decay_factor).floor();
        let below_pixel_fire_intensity: i8;
        if y == self.height - 1 {
            // bottommost layer gets input from constant "max intensity"
            let hotter_part = 4;
            let outside_center =
                x < self.width / hotter_part || x > self.width - self.width / hotter_part;
            let max_intensity = (self.palette.len() - 1) as i8;
            below_pixel_fire_intensity = if outside_center {
                // outside center = colder start
                // 1,2,3,4, 16,17,18
                // console::log_1(&format!("COLD = {}", x).into());
                max_intensity - 1 // ((self.rand_fn)() * 4.0).floor() as i8
            } else {
                max_intensity
            };
        } else {
            below_pixel_fire_intensity = self.intensity_data[x as usize][y as usize + 1] as i8;
        }

        let intensity = below_pixel_fire_intensity - decay as i8;
        let intensity = std::cmp::max(intensity, 0);

        // the right push inwards like this \
        // adding 1.0 to the division so we don't divide by <1.0 values
        let right_nudge: u8 = ((f32::from(y) / (1.0 + 4.0 * (self.rand_fn)())) + 1.0) as u8;
        // the left push inwards like this /
        let left_nudge = if right_nudge > self.height {
            0
        } else {
            self.height - right_nudge
        };

        let outside_boundaries = x + 11 < left_nudge || (x > 14) && (x - 14 > right_nudge);
        //let outside_boundaries = false;
        self.intensity_data[x as usize][y as usize] = if outside_boundaries {
            0
        } else {
            intensity as u8
        };
    }

    pub fn update_fire_intensity(&mut self) {
        for y in 0..self.height {
            for x in 1..(self.width - 1) {
                self.update_fire_pixel_intensity(x, y);
            }
        }
    }
}

#[cfg(not(target_vendor = "espressif"))]
pub fn rand() -> f32 {
    rand::random::<f32>()
}

const PANEL_W: u16 = 8;
const PANEL_H: u16 = 32;
// these screens zig-zag:
// | 00 01 02 03 04 |
// | 09 08 07 06 05 |
// | 10 11 12 13 14 |
pub fn pixmap(x: u16, y: u16) -> u16 {
    let panel = x / PANEL_W;
    let x_in_panel = x % PANEL_W;
    let is_updown = (panel % 2) == 0;

    let outrow = if is_updown { y } else { PANEL_H - y - 1 };
    let outcol = if is_updown {
        if (y % 2) == 0 {
            PANEL_W - x_in_panel - 1
        } else {
            x_in_panel
        }
    } else {
        if (y % 2) == 0 {
            PANEL_W - x_in_panel - 1
        } else {
            x_in_panel
        }
    };

    (panel * PANEL_W * PANEL_H) + outrow * PANEL_W + outcol
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut f = Fire::new(10, 8, 1.6, rand, &PALETTE);
        // for each call to update, there should be a new line of fire on top
        // there should never be any values in x=0 or x=max
        f.update_fire_intensity();
        f.update_fire_intensity();
        f.update_fire_intensity();
        for c in f.into_iter() {
            //println!("{:?}", c);
        }
    }
    #[test]
    fn map_pixmap_xy() {
        // first panel
        assert_eq!(pixmap(0, 0), 7);
        assert_eq!(pixmap(1, 0), 6);
        assert_eq!(pixmap(2, 0), 5);
        assert_eq!(pixmap(3, 0), 4);
        assert_eq!(pixmap(7, 0), 0);
        assert_eq!(pixmap(7, 1), 15);

        // second panel
        assert_eq!(pixmap(8, 0), 511);
        assert_eq!(pixmap(15, 0), 504);

        // third panel
        assert_eq!(pixmap(16, 0), 519);

        // y=1
        // first panel
        assert_eq!(pixmap(0, 1), 8);
        assert_eq!(pixmap(1, 1), 9);
        assert_eq!(pixmap(2, 1), 10);
        assert_eq!(pixmap(3, 1), 11);
        assert_eq!(pixmap(7, 1), 15);

        assert_eq!(pixmap(8, 1), 496);
        assert_eq!(pixmap(15, 1), 503);

        assert_eq!(pixmap(16, 1), 520);
    }
}
