static STR: &'static str = "test";

use std::ffi::CString;
use std::os::raw::c_char;

#[no_mangle]
pub extern "C" fn get_string(input: &str) -> *mut c_char {
    let s = CString::new(STR).unwrap();
    s.into_raw()
}

fn main() { }
