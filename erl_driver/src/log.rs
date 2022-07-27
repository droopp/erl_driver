//
// logger 
//
use std::time::SystemTime;

pub fn trace(msg: String) {
    let sys_time = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap();
    eprintln!("{:?}: {}\n", sys_time.as_millis(), msg);

}


