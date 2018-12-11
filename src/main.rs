use std::ffi::CString;
use std::os::raw::c_char;

#[no_mangle]
extern "C" {
    fn set_length(num: usize) -> ();
}

macro_rules! export_string {
    ($name:ident, $exec:expr) => {
        #[no_mangle]
        pub extern "C" fn $name() -> *mut c_char {
            let rust_string = $exec;
            unsafe { set_length(rust_string.len()); }
            let c_string = CString::new(rust_string)
                .expect("must be a valid C string");
            c_string.into_raw()
        }
    }
}


fn get_string() -> String {
    "test string".to_string()
}

export_string!(string_export, get_string());
export_string!(poop, "poop");

fn main() {
}
