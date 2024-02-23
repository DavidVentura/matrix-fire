#[macro_use]
extern crate lazy_static;

use render::{rand, Fire, PALETTE};
use std::sync::Mutex;

use wasm_bindgen::prelude::*;
use web_sys::console;

const W: u32 = 20;
const H: u32 = 16;

lazy_static! {
    static ref FIRE: Mutex<Fire<'static>> =
        Mutex::new(Fire::new(W as u8, H as u8, 1, 2.6, rand, &PALETTE, true));
}

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

    canvas.set_width(u32::from(W));
    canvas.set_height(u32::from(H));
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

    ctx.clear_rect(0.0, 0.0, W.into(), H.into());

    //console::log_1(&format!("Updating intensity").into());
    FIRE.lock().unwrap().update_fire_intensity();
    //console::log_1(&format!("Done Updating intensity").into());
    for c in FIRE.lock().unwrap().into_iter() {
        let val = format!("#{:02x}{:02x}{:02x}", c.c.r, c.c.g, c.c.b);
        //console::log_1(&format!("at x={}, y={}, value is {}", c.x, c.y, val).into());
        ctx.set_fill_style(&JsValue::from(val));
        ctx.fill_rect(c.x.into(), c.y.into(), 1.0, 1.0);
    }

    Ok(())
}
