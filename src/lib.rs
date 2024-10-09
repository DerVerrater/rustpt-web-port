
mod primitives;
mod scene;
mod renderer;

extern crate console_error_panic_hook;
use std::panic;

use primitives::{Vec2i, Vec2f, Vec3};
use renderer::{RenderProperties, Tile};
use scene::{Camera, Scene};
use wasm_bindgen::{prelude::*, Clamped};
use web_sys::ImageData;

/*
 * Renderer struct to represent the rendering machinery.
 * 
 * The JS code is meant to instantiate this, configure the render settings,
 * and then wait for the results.
 */
#[wasm_bindgen]
pub struct Renderer {
    aspect_ratio: f32,
    bounds: Vec2i,
    render_config: RenderProperties,
    scene: Scene,
    running: bool,
    signal_to_stop: bool, 
}

#[wasm_bindgen]
impl Renderer {
    pub fn new() -> Self {
        console_error_panic_hook::set_once();

        let aspect_ratio = 3.0 / 2.0;
        let image = Vec2i {
            x: 240,
            y: (240.0 / aspect_ratio) as i32
        };

        let render_config = RenderProperties {
            samples: 10,
            bounces: 50
        };

        // Scene (now includes camera)
        let scene = Scene {
            camera: Camera::new(
                Vec3::new(13.0, 2.0, 3.0), // lookfrom
                Vec3::zero(), // lookat
                Vec3::new(0.0, 1.0, 0.0), // vup
                20.0,
                aspect_ratio, 
                0.1, // aperture
                10.0, // dist_to_focus
            ),
            world: Scene::random_world()
        };

        return Self {
            aspect_ratio: 3.0 / 2.0,
            bounds: image,
            render_config,
            scene,
            running: false,
            signal_to_stop: false,
        }
    }

    pub fn start(&mut self, canvas_target: String) {
        // set running, clear signal (in case of restarting)
        self.running = true;
        self.signal_to_stop = false;
        let pixel_bytes = self.gogo();
        let window = web_sys::window().unwrap();
        let document = window.document().expect("Could not get document");
        let canvas = document
            .get_element_by_id(&canvas_target)
            .unwrap()
            .dyn_into::<web_sys::HtmlCanvasElement>()
            .expect("Could not get HTML Canvas Element");
        let context = canvas
            .get_context("2d")
            .expect("Could not get CanvasRenderingContext2d")
            .unwrap()
            .dyn_into::<web_sys::CanvasRenderingContext2d>()
            .expect("Could not convert CanvasRenderingContext2d into a dyn");
        let clamped_buf: Clamped<&[u8]> = Clamped(&pixel_bytes);
        let image_data_temp = ImageData::new_with_u8_clamped_array_and_sh(
            clamped_buf,
            self.bounds.x as u32,
            self.bounds.y as u32,
        ).expect("Could not create temporary ImageData from byte array");
        context.put_image_data(&image_data_temp, 0.0, 0.0)
            .expect("Could not put image on canvas element");
    }

    pub fn stop(&mut self) {
        self.signal_to_stop = true;
    }

    pub fn is_ready() -> bool {
        todo!();
    }
}

impl Renderer {
    // main rendering loop.
    fn gogo(&self) -> Vec<u8> {
        let mut pixel_bytes: Vec<u8> = Vec::new();
        for row in (0..self.bounds.y).rev() {
            let tile = Tile::render_line(row, self.bounds, &self.scene, &self.render_config);
            for pixel in tile.pixels {
                
                
                // gamma correction
                let scale = 1.0 / self.render_config.samples as f32;
                let r = (pixel.x * scale).sqrt();
                let g = (pixel.y * scale).sqrt();
                let b = (pixel.z * scale).sqrt();
                pixel_bytes.push((r * 255.0) as u8);
                pixel_bytes.push((g * 255.0) as u8);
                pixel_bytes.push((b * 255.0) as u8);
                pixel_bytes.push(255); // dummy alpha value to make ImageData happy
            }
        }
        return pixel_bytes;
    }
}

pub (crate) fn lerp(range: Vec2f, value: f32) -> f32 {
    return (1.0 - value) * range.x + value * range.y
}
