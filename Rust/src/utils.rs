use crate::utils::imports::import_alert;
use alloc::string::String;
use alloc::vec::Vec;
use core::mem;
use core::panic::PanicInfo;

mod imports;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[inline(always)]
pub fn alert(msg: *mut u8) {
    unsafe {
        import_alert(msg);
    }
}

#[inline(always)]
pub fn export_str(s: &str) -> *mut u8 {
    export_bytes(s.as_bytes())
}

#[inline(always)]
pub fn export_string(s: String) -> *mut u8 {
    export_bytes(s.as_bytes())
}

#[inline]
pub fn export_bytes(s: &[u8]) -> *mut u8 {
    let mut v = Vec::new();
    v.extend_from_slice(s);
    v.push(0);
    let ptr = v.as_mut_ptr();
    mem::forget(v);
    ptr
}

#[no_mangle]
pub extern "C" fn alloc(size: usize) -> *mut u8 {
    let mut buf = Vec::with_capacity(size);
    let ptr = buf.as_mut_ptr();
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
pub fn to_string(ptr: *mut u8) -> String {
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
pub fn to_vec(ptr: *mut u8, len: usize) -> Vec<u8> {
    unsafe {
        return core::slice::from_raw_parts(ptr, len as usize).to_vec();
    }
}
