
use wasm_bindgen::prelude::*;

const PPM_DATA: &'static str = include_str!("small.ppm");

// It's like an Iterator, but doesn't `impl Iterator ...`
// I'm not certain I can export a Rust std::Iterator through
// WASM, so I'm doing this nonsense.
#[wasm_bindgen]
struct ImageStepper {
    width: u32,
    height: u32,
    itr: std::str::Split<'static, &'static str>,
}

#[wasm_bindgen]
impl ImageStepper {
    pub fn new() -> Self {
        let mut itr = PPM_DATA.split("\n");
        // skip the first part. It's just the literal text "P3\n"They're metadata from the PPM file header.
        let _ = itr.next();
        let mut size = itr.next()
            .unwrap()
            .split(" ");
        let width: u32 = size.next()
            .unwrap()
            .parse()
            .unwrap();
        let height: u32  = size.next()
            .unwrap()
            .parse()
            .unwrap();

        ImageStepper { width, height, itr }
    }
    
    // I know that I'm loading an 80x80 image, but the program does not.
    // Yay for magic numbers
    pub fn width(&self) -> u32 { return self.width; }
    pub fn height(&self) -> u32 { return self.height; }
    
    pub fn get_next(&mut self) -> Option<String> {
        return Some(String::from(self.itr.next()?))
    }
}
