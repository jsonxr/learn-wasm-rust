import { add_one } from "wasm";
const result = add_one(41);
console.log('Answer to the life, the universe, and everything: ', result);
console.log(memory);

import { Image } from "wasm";
import { memory } from 'wasm/wasm_bg';

const img = new Image();
const ptr = img.pixels_ptr()
const pixels = new Uint8Array(memory.buffer, ptr, 8);
console.log(pixels);
