# wasm

Demonstrates how to get the raw memory from wasm and cast it to a useable structure in js without serialization.

https://egghead.io/lessons/javascript-load-a-webassembly-function-written-in-rust-and-invoke-it-from-javascript

# Interesting Files

## rust: `src/lib.rs`
```rust
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
```

## javascript: `www/index.js`
```javascript
import { Image } from "wasm";
import { memory } from 'wasm/wasm_bg';

const img = new Image();
const ptr = img.pixels_ptr()
const pixels = new Uint8Array(memory.buffer, ptr, 8);
console.log(pixels);
img.free();
```
# Build
```sh
bin/build
cd www
npm start
# After browser opens, look at console
```

