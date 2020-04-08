use wasm_bindgen::prelude::*;

// // A macro to provide `println!(..)`-style syntax for `console.log` logging.
#[macro_use]
macro_rules! log {
  ( $( $t:tt )* ) => {
      web_sys::console::log_1(&format!( $( $t )* ).into());
  }
}

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

mod color;
pub use color::*;
mod vertex;
pub use vertex::*;
mod image;
pub use image::*;
mod geometry;
pub use geometry::*;
mod buffer_attributes;
pub use buffer_attributes::*;

#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
  // This provides better error messages in debug mode.
  // It's disabled in release mode so it doesn't bloat up the file size.
  #[cfg(debug_assertions)]
  console_error_panic_hook::set_once();

  log!("main_js");
  Ok(())
}
