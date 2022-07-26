//
// Worker process 
//

use crate::config::Config;
use log::debug;
use std::{thread, time};


pub fn process(params: String, request: String) -> String  {


    let cfg = Config::new(params);

    debug!("params: {:?}", cfg.timeout);
    debug!("request: {:?}", request);
 
    if request == "error\n"{
        panic!("error occured");
    }

    // do work
    let sleep = time::Duration::from_millis(cfg.timeout);
    thread::sleep(sleep);


    return request;
}
