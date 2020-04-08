use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use js_sys::{Int8Array, Uint8Array, Int16Array, Uint16Array, Int32Array, Uint32Array, Float32Array, Float64Array};

use std::mem;


pub trait Attribute {
  fn get_size(&self) -> usize;
  fn get_len(&self) -> usize;
  unsafe fn get_ptr(&self) -> *const u32;
}
pub type BoxedAttribute = Box<dyn Attribute>;

pub struct TypedAttribute<T> {
  pub size: usize,
  pub values: Vec<T>,
}
impl<T> Attribute for TypedAttribute<T> {
  // fn get_ptr();
  fn get_len(&self) -> usize {
    return self.values.len();
  }
  fn get_size(&self) -> usize {
    return self.size;
  }
  // This is a pointer for javascript
  unsafe fn get_ptr(&self) -> *const u32 {
    let ptr = mem::transmute::<*const T, *const u32>(self.values.as_ptr());
    return ptr;
  }
}


// impl<T> TypedBufferAttribute<T> where T: BufferAttribute {
//   pub fn new(array: &[T], size: usize) -> TypedBufferAttribute<T> {
//     TypedBufferAttribute { 
//       item_size: size,
//       data: array.iter().cloned().collect(),
//     }
//   }
// }


type Int8Attribute = TypedAttribute<i8>;
impl Int8Attribute {
  fn from_jsvalue(js_values: JsValue, size: usize) -> BoxedAttribute {
    let mut values: Vec<i8> = Int8Array::from(js_values).to_vec();
    let attribute = Box::new(Int8Attribute { values, size });
    return attribute;
  }
}

type Uint8Attribute = TypedAttribute<u8>;
impl Uint8Attribute {
  fn from_jsvalue(js_values: JsValue, size: usize) -> BoxedAttribute {
    let mut values: Vec<u8> = Uint8Array::from(js_values).to_vec();
    let attribute = Box::new(Uint8Attribute { values, size });
    return attribute;
  }
}

//Uint8ClampedArray();
type Int16Attribute = TypedAttribute<i16>;
impl Int16Attribute {
  fn from_jsvalue(js_values: JsValue, size: usize) -> BoxedAttribute {
    let mut values: Vec<i16> = Int16Array::from(js_values).to_vec();
    let attribute = Box::new(Int16Attribute { values, size });
    return attribute;
  }
}

type Uint16Attribute = TypedAttribute<u16>;
impl Uint16Attribute {
  fn from_jsvalue(js_values: JsValue, size: usize) -> BoxedAttribute {
    let mut values: Vec<u16> = Uint16Array::from(js_values).to_vec();
    let attribute = Box::new(Uint16Attribute { values, size });
    return attribute;
  }
}

type Int32Attribute = TypedAttribute<i32>;
impl Int32Attribute {
  fn from_jsvalue(js_values: JsValue, size: usize) -> BoxedAttribute {
    let mut values: Vec<i32> = Int32Array::from(js_values).to_vec();
    let attribute = Box::new(Int32Attribute { values, size });
    return attribute;
  }
}

type Uint32Attribute = TypedAttribute<u32>;
impl Uint32Attribute {
  fn from_jsvalue(js_values: JsValue, size: usize) -> BoxedAttribute {
    let mut values: Vec<u32> = Uint32Array::from(js_values).to_vec();
    let attribute = Box::new(Uint32Attribute { values, size });
    return attribute;
  }
}

type Float32Attribute = TypedAttribute<f32>;
impl Float32Attribute {
  fn from_jsvalue(js_values: JsValue, size: usize) -> BoxedAttribute {
    let mut values: Vec<f32> = Float32Array::from(js_values).to_vec();
    let attribute = Box::new(Float32Attribute { values, size });
    return attribute;
  }
}

type Float64Attribute = TypedAttribute<f64>;
impl Float64Attribute {
  fn from_jsvalue(js_values: JsValue, size: usize) -> BoxedAttribute {
    let mut values: Vec<f64> = Float64Array::from(js_values).to_vec();
    let attribute = Box::new(Float64Attribute { values, size });
    return attribute;
  }
}


//BigInt64Array();
//BigUint64Array();






#[wasm_bindgen(js_name=BufferAttribute)]
pub struct WasmBufferAttribute {
  attribute: BoxedAttribute,
}
#[wasm_bindgen(js_class=BufferAttribute)]
impl WasmBufferAttribute {
  
  #[wasm_bindgen(constructor)]
  pub fn wasm_new(js_values: JsValue, size: usize) -> Result<WasmBufferAttribute, JsValue> {

    if js_values.has_type::<Int8Array>() {
      let attribute = Int8Attribute::from_jsvalue(js_values, size);
      return Ok(WasmBufferAttribute { attribute });
    } else if js_values.has_type::<Uint8Array>() {
      let attribute = Uint8Attribute::from_jsvalue(js_values, size);
      return Ok(WasmBufferAttribute { attribute });
    } else if js_values.has_type::<Int16Array>() {
      let attribute = Int16Attribute::from_jsvalue(js_values, size);
      return Ok(WasmBufferAttribute { attribute });
    } else if js_values.has_type::<Uint16Array>() {
      let attribute = Uint16Attribute::from_jsvalue(js_values, size);
      return Ok(WasmBufferAttribute { attribute });
    } else if js_values.has_type::<Int32Array>() {
      let attribute = Int32Attribute::from_jsvalue(js_values, size);
      return Ok(WasmBufferAttribute { attribute });
    } else if js_values.has_type::<Uint32Array>() {
      let attribute = Uint32Attribute::from_jsvalue(js_values, size);
      return Ok(WasmBufferAttribute { attribute });
    } else if js_values.has_type::<Float32Array>() {
      let attribute = Float32Attribute::from_jsvalue(js_values, size);
      return Ok(WasmBufferAttribute { attribute });
    } else if js_values.has_type::<Float64Array>() {
      let attribute = Float64Attribute::from_jsvalue(js_values, size);
      return Ok(WasmBufferAttribute { attribute });
    }

    return Err(JsValue::from_str("new BufferAttribute(...What are you passing in?)"));
  }

  #[wasm_bindgen(js_name=getPtr)]
  pub fn get_ptr(&self) -> *const u32 {
    unsafe { return self.attribute.get_ptr() }
  }
  #[wasm_bindgen(js_name=getCount)]
  pub fn get_len(&self) -> usize {
    return self.attribute.get_len();
  }
  #[wasm_bindgen(js_name=getSize)]
  pub fn get_size(&self) -> usize {
    return self.attribute.get_size();
  }
}

