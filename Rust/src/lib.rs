#![no_std]
#![feature(alloc_error_handler)]
#![feature(link_llvm_intrinsics)]
extern crate alloc;
extern crate wee_alloc;

use alloc::string::String;
use core::alloc::Layout;

use crate::utils::{alert, export_str, export_string, to_string};

mod utils;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[no_mangle]
pub extern "C" fn greet2(ptr: *const u8) -> *const u8 {
    let mut string_content = String::from("Hello, ");

    string_content.push_str(to_string(ptr).as_str());
    string_content.push_str("!");

    return export_string(string_content);
}

#[no_mangle]
pub extern "C" fn greet() {
    alert(export_str("Hello Chris!"));
}

#[alloc_error_handler]
fn allocation_error(_: Layout) -> ! {
    panic!("Out of memory!");
}
