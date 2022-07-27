//
// Worker process 
//

use crate::config::Config;
use crate::log::trace;
use std::{thread, time};


pub fn process(params: String, request: String) -> String  {


    let cfg = Config::new(params);

    trace(format!("params: {:?}", cfg.timeout));
    trace(format!("request: {:?}", request));
 
    if request == "error\n"{
        panic!("error occured");
    }

    // do work
    let sleep = time::Duration::from_millis(cfg.timeout);
    thread::sleep(sleep);


    return request;
}
