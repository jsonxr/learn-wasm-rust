mod utils;
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// // A macro to provide `println!(..)`-style syntax for `console.log` logging.
macro_rules! log {
  ( $( $t:tt )* ) => {
      web_sys::console::log_1(&format!( $( $t )* ).into());
  }
}

#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
  // This provides better error messages in debug mode.
  // It's disabled in release mode so it doesn't bloat up the file size.
  #[cfg(debug_assertions)]
  console_error_panic_hook::set_once();

  log!("main_js");
  Ok(())
}

#[wasm_bindgen]
pub fn add_one(x: u32) -> u32 {
  log!("add_one x:{}", x);
  x + 1
}

#[wasm_bindgen]
pub struct Color {
  r: u8,
  g: u8,
  b: u8,
  a: u8,
}

#[wasm_bindgen]
pub struct Image {
  pixels: Vec<Color>,
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
    self.pixels.as_ptr()
  }
}
