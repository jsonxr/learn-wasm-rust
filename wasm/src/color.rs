use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Debug,Copy,Clone)]
pub struct Color {
  pub r: u8,
  pub g: u8,
  pub b: u8,
  pub a: u8,
}
