use web_sys::console;

use wasm_bindgen::JsValue;

pub mod color;

pub fn print(message: &str){
    console::log_1(&JsValue::from_str(message));
}