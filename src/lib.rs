extern crate wasm_bindgen;
extern crate ndarray;

use wasm_bindgen::prelude::*;

mod utils;

//
//
//
#[wasm_bindgen]
pub struct AutoMaster {
}

#[wasm_bindgen]
impl AutoMaster {
    pub fn new() {
        utils::set_panic_hook();
    }

    pub fn select_as_ref_audio() {
    }

    pub fn select_as_target_audio() {
    }
}
