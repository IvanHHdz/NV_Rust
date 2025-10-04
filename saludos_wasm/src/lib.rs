use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn greet() {
    println!("¡Hola, mundo desde WebAssembly!");
}