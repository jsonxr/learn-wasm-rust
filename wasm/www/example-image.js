import { Image, BufferGeometry } from "wasm";
import { memory } from 'wasm/wasm_bg';

const img = new Image();
const ptr = img.pixels_ptr()
console.log('ptr: ', `0x${ptr.toString(16)}`);
const pixels = new Uint8Array(memory.buffer, ptr, 8);
console.log(ptr);
console.log(pixels);

// console.log(img.box_ptr());
// const values = new Uint8Array(memory.buffer, img.box_ptr(), 8);
// console.log(values);
