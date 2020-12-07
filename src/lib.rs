use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
pub fn res()->String{
    format!("Rust側の返答")
}

#[wasm_bindgen]
pub fn read_js_fnction(){
    //jsのアラート
    alert("call JavaScript function".to_string());
    //js関数での処理
    write_p_text("Rust call function".to_string());
}

#[wasm_bindgen(module="/js-file/js-code-file.js")]
extern "C"{
    fn write_p_text(a:String);
}

#[wasm_bindgen]
extern "C"{
    fn alert(text:String);
}