mod utils;

use wasm_bindgen::prelude::*;
use crate::utils::set_panic_hook;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn setup() {
    set_panic_hook();
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, NateLu!");
}
