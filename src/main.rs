use std::ffi::CString;
use std::os::raw::c_char;

macro_rules! export_string {
    ($name:ident, $exec:expr) => {
        #[no_mangle]
        pub extern "C" fn $name(get_len: bool) -> usize {
            if get_len {
                $exec.len()
            } else {
                let s = CString::new($exec)
                    .expect("must be a valid C string");
                s.into_raw() as *mut c_char as usize
            }
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
