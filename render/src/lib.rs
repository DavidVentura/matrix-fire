use rgb::RGB8;

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

pub struct Fire<'a> {
    width: u8,
    height: u8,
    upscale_factor: u8,
    decay_factor: f32,
    rand_fn: fn() -> f32,
    palette: &'a [RGB8],
    intensity_data: Vec<Vec<u8>>,
    curr_x: usize,
    curr_y: usize,
}

#[derive(Debug)]
pub struct ColorCoord {
    pub c: RGB8,
    pub x: u8,
    pub y: u8,
}

impl<'a> Iterator for &mut Fire<'a> {
    type Item = ColorCoord;

    fn next(&mut self) -> Option<Self::Item> {
        // TODO flippy X
        let ret = if self.curr_y == (self.height as usize) {
            self.curr_y = 0;
            self.curr_x = 0;
            None
        } else {
            let firstbit = &self.intensity_data[self.curr_x];
            let c = self.palette[firstbit[self.curr_y] as usize]; // self.curr_y
            Some(ColorCoord {
                c,
                x: self.curr_x as u8,
                y: self.curr_y as u8,
            })
        };

        self.curr_x += 1;
        if self.curr_x == self.width as usize {
            self.curr_y += 1;
            self.curr_x = 0;
        }
        ret
    }
}

impl<'a> Fire<'a> {
    pub fn new(
        width: u8,
        height: u8,
        upscale_factor: u8,
        decay_factor: f32,
        rand_fn: fn() -> f32,
        palette: &'a [RGB8],
    ) -> Fire<'a> {
        Fire {
            width,
            height,
            upscale_factor,
            decay_factor,
            rand_fn,
            palette,
            intensity_data: vec![vec![0; height as usize]; width as usize],
            curr_x: 0,
            curr_y: 0,
        }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        // rand::random::<f32>()
        let mut f = Fire::new(10, 8, 1, 1.6, rand, &PALETTE);
        // for each call to update, there should be a new line of fire on top
        // there should never be any values in x=0 or x=max
        f.update_fire_intensity();
        f.update_fire_intensity();
        f.update_fire_intensity();
        for c in &mut f {
            println!("{:?}", c);
        }
    }
}
