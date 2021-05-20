use std::os::raw::*;


extern "C" {
    pub fn xmrig_main(argc: c_int, argv: *mut *mut c_char) -> c_int;
}

/// Starts `xmrig` exactly as it would be run on the command-line without any arguments
pub fn start() {
    unsafe {
        xmrig_main(0i32, std::ptr::null_mut());
    }
}

fn main() {
    start();
}