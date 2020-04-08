import { memory } from 'wasm/wasm_bg';
import { BufferAttribute, BufferGeometry} from "wasm";
//import { BufferAttribute, WasmBufferGeometry as BufferGeometry} from "wasm";
//const geometry = new BufferGeometry();

const pos = new BufferAttribute(new Float32Array( [
  -1.0, -1.0,  1.0,
	1.0,  -1.0,  1.0,
	1.0,   1.0,  1.0,

	1.0,   1.0,  1.0,
	-1.0,  1.0,  1.0,
	-1.0, -1.0,  1.0
]), 3);
const posPtr = pos.getPtr();
const posArray = new Float32Array(memory.buffer, posPtr, pos.getCount());
console.log(posArray);

//geometry.setAttribute('position', positions);

const colors = new BufferAttribute(new Uint16Array( [
  255, 0, 0,
  0, 255, 0,
  0, 0, 255,
]), 3);
const colorsPtr = colors.getPtr();
const colorsArray = new Uint16Array(memory.buffer, colorsPtr, colors.getCount());
console.log(colorsArray);


// geometry.setAttribute('color', colors);

// geometry.free();
