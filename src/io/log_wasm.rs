use wasm_bindgen::prelude::*;
use web_sys::console;

pub fn log(s: &str) {
    let s2: JsValue = s.into();
    console::log_1(&s2);
}
