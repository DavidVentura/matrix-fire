pub const WIDTH: u16 = 20;
pub const HEIGHT: u16 = 16;
const DECAY_FACTOR: f32 = 2.6;
static PALETTE: [u32; 11] = [
    0x000000, 0x311e10, 0x573318, 0x76411e, 0xd14b02, 0xde9a1a, 0xff9c17, 0xfeb417, 0xffcf19,
    0xfeed18, 0xfaff4a,
];

pub fn update_fire_intensity(pix: &mut [u8]) {
    _update_fire_intensity(pix);
}
fn _update_fire_intensity(pix: &mut [u8]) {
    for y in 0..HEIGHT {
        for x in 1..(WIDTH - 1) {
            let idx: u16 = x + y * WIDTH;
            update_fire_pixel_intensity(x, y, idx, pix);
        }
    }
}

fn update_fire_pixel_intensity(x: u16, y: u16, idx: u16, pix: &mut [u8]) {
    let below_pixel_index = idx + WIDTH as u16;

    let decay = (rand::random::<f32>() * DECAY_FACTOR).floor();
    let below_pixel_fire_intensity: i8;
    if y == HEIGHT - 1 {
        // bottommost layer gets input from constant "max intensity"
        let hotter_part = 4;
        let outside_center = x < WIDTH / hotter_part || x > WIDTH - WIDTH / hotter_part;
        let max_intensity = (PALETTE.len() - 1) as i8;
        below_pixel_fire_intensity = if outside_center {
            // outside center = colder start
            // 1,2,3,4, 16,17,18
            // console::log_1(&format!("COLD = {}", x).into());
            max_intensity - (rand::random::<f32>() * 4.0).floor() as i8
        } else {
            max_intensity
        };

        /*
        console::log_1(&format!("intensity is {}", below_pixel_fire_intensity).into());
        */
    } else {
        below_pixel_fire_intensity = pix[below_pixel_index as usize] as i8;
    }

    let intensity = below_pixel_fire_intensity - decay as i8;
    let intensity = std::cmp::max(intensity, 0);

    // the right push inwards like this \
    // adding 1.0 to the division so we don't divide by <1.0 values
    let right_nudge: u16 = ((f32::from(y) / (1.0 + 4.0 * rand::random::<f32>())) + 1.0) as u16;
    // the left push inwards like this /
    let left_nudge = if right_nudge > HEIGHT {
        0
    } else {
        HEIGHT - right_nudge
    };

    let outside_boundaries = x + 11 < left_nudge || (x > 14) && (x - 14 > right_nudge);
    pix[idx as usize] = if outside_boundaries {
        0
    } else {
        intensity as u8
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut pix: &mut [u8] = &mut [0; WIDTH as usize * HEIGHT as usize];
        update_fire_intensity(&mut pix);
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                let idx: u16 = x + y * WIDTH;
                let val = pix[idx as usize];
                if y < HEIGHT - 1 {
                    assert!(val == 0);
                } else {
                    if x > 1 && x < WIDTH - 2 {
                        // the edge 2 pixels are random
                        assert!(val != 0);
                    }
                }
            }
        }
    }
}
