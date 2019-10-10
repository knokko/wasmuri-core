use web_sys::console;

use wasm_bindgen::JsValue;

pub fn print(message: &str){
    console::log_1(&JsValue::from_str(message));
}

mod region;
pub use region::*;