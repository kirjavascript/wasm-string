use std::ffi::CString;
use std::os::raw::c_char;

#[no_mangle]
extern "C" {
    fn stack_push(num: usize);
    fn console_log_stack();
}

#[no_mangle]
pub extern "C" fn deallocate_string(ptr: *mut c_char) {
    unsafe { let _ = CString::from_raw(ptr); }
}

macro_rules! export_string {
    ($name:ident, $exec:expr) => {
        #[no_mangle]
        pub extern "C" fn $name() {
            let rust_string = $exec;
            let rust_string_length = rust_string.len();
            let c_string = CString::new(rust_string)
                .expect("must be a valid C string");
            unsafe { stack_push(rust_string_length); }
            unsafe { stack_push(c_string.into_raw() as *mut c_char as usize); }
        }
    }
}

pub fn console_log(string: &str) {
    for c in string.chars() {
        unsafe { stack_push(c as usize); }
    }
    unsafe { console_log_stack(); }
}


fn get_string() -> &'static str {
    "test string"
}

export_string!(string_export, get_string());

fn main() {
}
