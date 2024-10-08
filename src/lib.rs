
use wasm_bindgen::prelude::*;

const PPM_DATA: &'static str = include_str!("small.ppm");

// It's like an Iterator, but doesn't `impl Iterator ...`
// I'm not certain I can export a Rust std::Iterator through
// WASM, so I'm doing this nonsense.
#[wasm_bindgen]
struct ImageStepper {
    itr: std::str::Split<'static, &'static str>,
}

#[wasm_bindgen]
impl ImageStepper {
    pub fn new() -> Self {
        let mut itr = PPM_DATA.split("\n");
        // skip the first 3 parts. They're metadata from the PPM file header.
        let _ = itr.next();
        let _ = itr.next();
        let _ = itr.next();

        ImageStepper { itr }
    }
    
    // I know that I'm loading an 80x80 image, but the program does not.
    // Yay for magic numbers
    pub fn width(&self) -> u32 { return 80; }
    pub fn height(&self) -> u32 { return 80; }
    
    pub fn get_next(&mut self) -> Option<String> {
        return Some(String::from(self.itr.next()?))
    }
}
