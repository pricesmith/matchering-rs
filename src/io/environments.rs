#![feature(proc_macro, wasm_custom_section, wasm_import_module)]
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern {
    type Function;

    #[wasm_bindgen(constructor)]
    fn new() -> Function();
}

pub fn app_env() -> bool {
    // test for js environments
    if Function::new("try {return this===window;}catch(e){return false;}") {
        "browser"
    } else if Function::new("try {return this===global;}catch(e){return false;}") {
        "node"
    }
}


