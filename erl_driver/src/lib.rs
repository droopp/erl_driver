mod func;

use std::ffi::CStr;
use std::os::raw::c_char;
use std::ffi::CString;

use log::trace;
use std::panic;


#[no_mangle]
pub extern "C" fn port_call(request: *const c_char, len: usize) -> *mut c_char {

    //env_logger::try_init().expect("..");

    // request
    let raw_req = unsafe { CStr::from_ptr(request).to_string_lossy().to_owned() };

    let req: &str = &raw_req[..len];

    trace!("read data from port c str: {}", req);

    // process
    // fn (req) -> output
    let result = panic::catch_unwind(|| {
            func::process(String::from(req))
    });

    let output =  match result {
        Ok(value) => value,
        Err(_) => "error".to_string()
    };

    trace!("send data to port: {}", output);

    // response
    let res = CString::new(output).expect("CString::new failed");
    res.into_raw()
}
