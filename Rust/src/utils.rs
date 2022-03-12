use alloc::string::String;
use alloc::vec::Vec;
use core::mem;
use core::panic::PanicInfo;

use crate::utils::imports::{import_alert, import_trap};

mod imports;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    unsafe {
        import_trap();
    }
    loop {}
}

#[inline(always)]
pub fn alert(msg: *const u8) {
    unsafe {
        import_alert(msg);
    }
}

#[inline(always)]
pub fn export_str(s: &str) -> *const u8 {
    export_str_bytes(s.as_bytes())
}

#[inline(always)]
pub fn export_string(s: String) -> *const u8 {
    export_str_bytes(s.as_bytes())
}

#[inline]
pub fn export_str_bytes(s: &[u8]) -> *const u8 {
    let mut v = Vec::with_capacity(s.len() + 1);
    v.extend_from_slice(s);
    v.push(0);
    let ptr = v.as_mut_ptr();
    mem::forget(v);
    ptr
}

/*#[inline]
pub fn export_byte_vec(v: &mut Vec<u8>) -> *const u8 {
    let ptr = v.as_ptr();
    mem::forget(v);
    ptr
}*/

#[no_mangle]
pub extern "C" fn alloc(size: usize) -> *const u8 {
    let buf = Vec::with_capacity(size);
    let ptr = buf.as_ptr();
    mem::forget(buf);
    ptr
}

#[no_mangle]
pub extern "C" fn dealloc(ptr: *mut u8, size: usize) {
    unsafe {
        let _ = Vec::from_raw_parts(ptr, 0, size);
    }
}

#[inline]
pub fn to_string(ptr: *const u8) -> String {
    let mut len = 0;
    unsafe {
        while *ptr.offset(len) != 0 {
            len += 1;
        }
        let slice = to_vec(ptr, len as usize);
        String::from_utf8_unchecked(slice)
    }
}

#[inline]
pub fn to_vec(ptr: *const u8, len: usize) -> Vec<u8> {
    unsafe {
        return core::slice::from_raw_parts(ptr, len as usize).to_vec();
    }
}
