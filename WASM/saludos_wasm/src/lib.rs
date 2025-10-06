use wasm_bindgen::prelude::*;
use web_sys;

#[wasm_bindgen]
pub fn greet() -> String {
    web_sys::console::log_1(&"¡Hola, mundo desde WebAssembly!".into());
    format!("¡Hola, mundo desde WebAssembly!")
}