mod func;
mod config;
mod log;

use std::ffi::CStr;
use std::os::raw::c_char;
use std::ffi::CString;

use log::trace;
use std::panic;
use std::env;

#[no_mangle]
pub extern "C" fn port_call(request: *const c_char, len: usize) -> *mut c_char {

    // request
    let raw_req = unsafe { CStr::from_ptr(request).to_string_lossy().to_owned() };

    let req: &str = &raw_req[..len];

    trace(format!("read data from port c str: {}", req));

    let lib_name = module_path!().split("::").next().unwrap().to_uppercase();

    // process
    // fn (params, req) -> output
     
    let result = panic::catch_unwind(|| {

            func::process(env::var(lib_name).unwrap_or("".to_string()), 
                          String::from(req))
    });

    let output =  match result {
        Ok(value) => value,
        Err(_) => "error".to_string()
    };

    trace(format!("send data to port: {}", output));

    // response
    let res = CString::new(output).expect("CString::new failed");
    res.into_raw()
}
