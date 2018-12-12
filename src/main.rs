use std::ffi::CString;
use std::os::raw::c_char;

// imported functions

#[no_mangle]
extern "C" {
    fn stack_push(num: usize);
    fn console_log_stack();
}

// importing strings from JS

#[no_mangle]
pub unsafe extern "C" fn alloc_js_string(cap: usize) -> JSString {
    let mut d = Vec::with_capacity(cap);
    d.set_len(cap);
    let s = Box::new(String::from_utf8_unchecked(d));
    JSString(Box::into_raw(s))
}

#[no_mangle]
pub unsafe extern "C" fn get_mut_js_string(string: JSString) -> *mut u8 {
    (&mut *string.0).as_mut_vec().as_mut_ptr()
}

#[repr(transparent)]
pub struct JSString(pub *mut String);

impl JSString {
    fn to_owned(self) -> Box<String> {
        unsafe { Box::from_raw(self.0) } // dealloc js string
    }
}

// exports to js

#[no_mangle]
pub extern "C" fn dealloc_rust_string(ptr: *mut c_char) {
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

export_string!(TEST_STRING, get_test_string());

fn get_test_string() -> &'static str {
    "test string"
}

#[no_mangle]
pub extern "C" fn receive_string(string: JSString) {
    console_log(&format!("string came from rust: {:?}", &string.to_owned()));
}


fn main() { }
