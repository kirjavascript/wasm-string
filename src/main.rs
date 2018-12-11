use std::ffi::CString;
use std::os::raw::c_char;

// imports from js

#[no_mangle]
extern "C" {
    fn stack_push(num: usize);
    fn console_log_stack();
}

// https://stackoverflow.com/questions/49014610/passing-a-javascript-string-to-a-rust-function-compiled-to-webassembly/49020435

// exports to js

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
            let c_string = std::ffi::CString::new(rust_string)
                .expect("must be a valid C string");
            unsafe { stack_push(rust_string_length); }
            unsafe { stack_push(c_string.into_raw() as usize); }
        }
    }
}

// helpers

pub fn console_log(string: &str) {
    #[cfg(target_arch = "wasm32")]
    {
        for c in string.chars() {
            unsafe { stack_push(c as usize); }
        }
        unsafe { console_log_stack(); }
    }
}

// examples

export_string!(TEST_STRING, get_string());
fn get_string() -> &'static str {
    "test string"
}

fn main() { }
