use wasm_bindgen::prelude::*;
use super::Color;

#[wasm_bindgen]
pub struct Image {
  #[wasm_bindgen(skip)]
  pub pixels: Vec<Color>,
  //pub Box<Vec<Color>>,
}

#[wasm_bindgen]
impl Image {
  #[wasm_bindgen(constructor)]
  pub fn new() -> Image {
    let color1 = Color { r: 60, g: 70, b: 90, a: 1 };
    let color2 = Color { r: 255, g: 0, b: 0, a: 1 };
    let pixels = vec![color1, color2];
    Image {
      pixels,
    }
  }

  #[wasm_bindgen]
  pub fn pixels_ptr(&self) -> *const Color {
    let ptr = self.pixels.as_ptr();
    log!("rust ptr: {:p}", ptr);
    ptr
    
  }

  pub fn box_ptr(&self) -> *const u32 {
    let i: u32 = 42;
    return &i;
  }
}
