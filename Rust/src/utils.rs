use wasm_bindgen::prelude::*;
use std::panic;

#[wasm_bindgen]
extern {
    #[wasm_bindgen(js_name = error, js_namespace = console)]
    fn error_str(msg: String);

    #[wasm_bindgen(js_name = error, js_namespace = console)]
    fn error(msg: &str);

    #[wasm_bindgen(js_namespace = console)]
    fn log(msg: &str);
}

fn hook_impl(info: &panic::PanicInfo) {
    error_str(info.to_string());
}

#[inline]
pub fn set_panic_hook() {
    panic::set_hook(Box::new(hook_impl));
}