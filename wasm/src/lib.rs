use wasm_bindgen::prelude::*;
use web_sys::console;

// todo 2d matrix, non-static
static mut PIXELS_IDX: &'static mut [u8] = &mut [0; WIDTH as usize * HEIGHT as usize];
use crate::{HEIGHT, PALETTE, WIDTH};

#[wasm_bindgen(start)]
pub fn run() -> Result<(), JsValue> {
    console_error_panic_hook::set_once();
    setup_canvas();
    Ok(())
}

fn setup_canvas() {
    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document.get_element_by_id("canvas").unwrap();
    let canvas: web_sys::HtmlCanvasElement = canvas
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .map_err(|_| ())
        .unwrap();

    canvas.set_width(u32::from(WIDTH));
    canvas.set_height(u32::from(HEIGHT));
}

#[wasm_bindgen]
pub fn render_fire() -> Result<(), JsValue> {
    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document.get_element_by_id("canvas").unwrap();
    let canvas: web_sys::HtmlCanvasElement = canvas
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .map_err(|_| ())
        .unwrap();

    let ctx = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap();

    ctx.clear_rect(0.0, 0.0, WIDTH.into(), HEIGHT.into());
    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            let idx = x + y * WIDTH;
            let fire_intensity = unsafe { PIXELS_IDX[idx as usize] };

            let val = format!("#{:06x}", PALETTE[fire_intensity as usize]);
            // console::log_1(&format!("at x={}, y={}, value is {}", x, y, val).into());
            ctx.set_fill_style(&JsValue::from(val));
            ctx.fill_rect(x.into(), y.into(), 1.0, 1.0);
        }
    }

    Ok(())
}

// TODO extract random to u32
#[wasm_bindgen]
pub fn update_fire_intensity() {
    crate::_update_fire_intensity(unsafe { PIXELS_IDX });
}
