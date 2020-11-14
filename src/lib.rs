use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
pub fn res()->String{
    format!("Rust側の返答")
}