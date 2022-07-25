mod func;

use std::ffi::CStr;
use std::os::raw::c_char;
use std::ffi::CString;

use log::trace;


#[no_mangle]
pub extern "C" fn port_call(request: *const c_char, len: usize) -> *mut c_char {

    env_logger::init();

    // request
    let raw_req = unsafe { CStr::from_ptr(request).to_string_lossy().to_owned() };

    let req: &str = &raw_req[..len];

    trace!("read data from port c str: {}", req);

    // process
    // fn (req) -> output
    let output = func::process(String::from(req));

    trace!("send data to port: {}", output);

    // response
    let res = CString::new(output).expect("CString::new failed");
    res.into_raw()
}
