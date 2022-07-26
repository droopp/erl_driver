mod func;
mod config;

use std::ffi::CStr;
use std::os::raw::c_char;
use std::ffi::CString;

use log::trace;
use std::panic;
use std::env;

use std::cell::RefCell;
 
thread_local!(static IS_INIT: RefCell<bool> = RefCell::new(false));


#[no_mangle]
pub extern "C" fn port_call(request: *const c_char, len: usize) -> *mut c_char {

    // check global logger flag
    IS_INIT.with(|is_init| {
        if *is_init.borrow() == false{
            env_logger::init();
            *is_init.borrow_mut() = true;
        }
    });

    // request
    let raw_req = unsafe { CStr::from_ptr(request).to_string_lossy().to_owned() };

    let req: &str = &raw_req[..len];

    trace!("read data from port c str: {}", req);

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

    trace!("send data to port: {}", output);

    // response
    let res = CString::new(output).expect("CString::new failed");
    res.into_raw()
}
