//
// Worker process 
//

use log::debug;

pub fn process(request: String) -> String {

    debug!("request: {:?}", request);
    return request;

}
