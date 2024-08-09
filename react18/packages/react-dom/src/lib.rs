use wasm_bindgen::prelude::*;

#[wasm_bindgen(js_name = createRoot)]
pub fn create_root(_container: &JsValue) -> JsValue {
    JsValue::null()
}
