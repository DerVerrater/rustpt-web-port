
use wasm_bindgen::prelude::*;

const PPM_DATA: &'static str = "one two three four five";

// It's like an Iterator, but doesn't `impl Iterator ...`
// I'm not certain I can export a Rust std::Iterator through
// WASM, so I'm doing this nonsense.
#[wasm_bindgen]
struct ImageStepper {
    idx: usize,
}

#[wasm_bindgen]
impl ImageStepper {
    pub fn new() -> Self {
        ImageStepper { idx: 0 }
    }
    
    pub fn get_next(&mut self) -> Option<String> {
        self.idx += 1;
        let slice = PPM_DATA.get(self.idx..(self.idx+1))?;
        return Some(String::from(slice))
    }

    pub fn has_more(&self) -> bool {
        self.idx >= PPM_DATA.len()
    }
}