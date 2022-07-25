//
// Worker process 
//

use log::debug;

pub fn process(request: String) -> String  {

    debug!("request: {:?}", request);
 
    if request == "error\n"{
        panic!("error occured");
    }

   return request;

}
