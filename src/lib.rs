// Current prelude for using `wasm_bindgen`, and this'll get smaller over time!
#![feature(proc_macro, wasm_custom_section, wasm_import_module)]
use wasm_bindgen::prelude::*;

pub mod io;
pub mod _utils;
pub mod types;
pub mod utils;
pub mod defaults;
pub mod dsp;
pub mod statsmodel;
pub mod stage_helpers;
pub mod limiter;


// capture and implement all needed js functions here
#[wasm_bindgen]
extern {
    // fn [add js stuff here]
}

// custom wasm function using cherry picked js functions
#[wasm_bindgen]
pub fn greet() {
    alert("Hello, test-wasm!");
}

#[wasm_bindgen]
pub struct AutoMaster {
}

#[wasm_bindgen]
impl AutoMaster {
    pub fn new() {
        _utils::set_panic_hook();
    }

    pub fn select_as_ref_audio() {
    }

    pub fn select_as_target_audio() {
    }
}
