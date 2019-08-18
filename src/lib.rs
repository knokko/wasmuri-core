use web_sys::{
    HtmlCanvasElement,
    WebGlRenderingContext
};

use wasm_bindgen::{
    JsCast,
    JsValue
};

use serde::ser::{
    Serialize,
    Serializer,
    SerializeMap
};

pub mod util;

pub fn get_gl(canvas: &HtmlCanvasElement) -> WebGlRenderingContext {
    let gl = canvas.get_context_with_context_options("webgl", &JsValue::from_serde(&ContextJSON{}).expect("Should be able to serialize context options"));
    gl.expect("get_context('webgl') should not fail (1)").expect("get_context('webgl') should not fail (2)").dyn_into::<WebGlRenderingContext>()
    .expect("The webgl context should be an instance of WebGlRenderingContext")
}

struct ContextJSON {}

impl Serialize for ContextJSON {

    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        let mut map = serializer.serialize_map(Some(1))?;
        map.serialize_entry("preserveDrawingBuffer", &true)?;
        map.end()
    }
}