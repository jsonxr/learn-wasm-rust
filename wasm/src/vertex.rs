use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Debug,Copy,Clone)]
pub struct Vertex {
  pub x: f32,
  pub y: f32,
  pub z: f32,
}


#[wasm_bindgen]
impl Vertex {
  #[wasm_bindgen(constructor)]
  pub fn new(x: f32, y: f32, z: f32) -> Vertex {
    Vertex {x, y, z}
  }
}