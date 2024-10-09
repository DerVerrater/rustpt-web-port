
extern crate console_error_panic_hook;
use std::panic;

use wasm_bindgen::{prelude::*, Clamped};
use web_sys::ImageData;


/*
 * Renderer struct to represent the rendering machinery.
 * 
 * The JS code is meant to instantiate this, configure the render settings,
 * and then wait for the results.
 */
#[wasm_bindgen]
pub struct Renderer {}

#[wasm_bindgen]
impl Renderer {
    pub fn new() -> Self {
        console_error_panic_hook::set_once();
        Self {}
    }

    pub fn start() {
        todo!();
    }

    pub fn stop() {
        todo!();
    }

    pub fn is_ready() -> bool {
        todo!();
    }
}

impl Renderer {
    // main rendering loop.
    fn gogo() {
        todo!();
    }
}

const PPM_DATA: &'static str = include_str!("small.ppm");

#[wasm_bindgen]
pub async fn paint_image(canvas_name: String) -> Result<(), JsValue> {

    let vec_bytes = ppm_to_u8(&PPM_DATA);
    
    let window = web_sys::window().unwrap();
    let document = window.document().expect("Could not get document");
    let canvas = document
        .get_element_by_id(&canvas_name)
        .unwrap()
        .dyn_into::<web_sys::HtmlCanvasElement>()?;
    let context = canvas
        .get_context("2d")?
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()?;
    let clamped_buf: Clamped<&[u8]> = Clamped(&vec_bytes);
    let image_data_temp = ImageData::new_with_u8_clamped_array_and_sh(clamped_buf, 80, 53)?;
    context.put_image_data(&image_data_temp, 0.0, 0.0)?;
    Ok(())
}

fn ppm_to_u8<'input>(input: &'input str) -> Vec<u8> {
    let mut ppm_iter = input.split("\n");
    let _ = ppm_iter.next();    // consume "P3"
    let _ = ppm_iter.next();    // consume width & height
    let _ = ppm_iter.next();    // consume max value of each color (255)

    let mut bytes: Vec<u8> = Vec::new();
    for line in ppm_iter {
        for component in line.split(" ") {
            if let Ok(value) = component.parse::<u8>() {
                bytes.push(value);
            }
        }
        bytes.push(255);
    }
    // the last line is empty. I'm not going to fix the logic to avoid
    // pushing an extra alpha value, I'll just do this garbage.
    // It's not supposed to be a text parser anyway. Fight me.
    bytes.pop();
    return bytes;
}
