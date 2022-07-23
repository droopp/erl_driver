mod func;

use std::ffi::CStr;
use std::os::raw::c_char;
use std::ffi::CString;

#[no_mangle]
pub extern "C" fn port_call(request: *const c_char, len: usize) -> *mut c_char {

    // request
    let raw_req = unsafe { CStr::from_ptr(request).to_string_lossy().to_owned() };
    let req: &str = &raw_req[..len];

    // process
    // fn (req) -> output

    let output = func::process(String::from(req));

    // response
    let res = CString::new(output).expect("CString::new failed");
    res.into_raw()
}
