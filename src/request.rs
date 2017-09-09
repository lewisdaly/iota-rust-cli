/**
 * Make the api request
 */

//TODO: make a sandbox client if need be, and implement the Request trait!
pub struct Client {
    pub host: &'static str,
    pub port: i32
}

pub trait Request {
    //This will also have to take a generic command
    fn make_request(&self) -> bool;
}

impl Request for Client {
    //Implement this request
    fn make_request(&self) -> bool {
        false
    }
}


//TODO: we can also implement a sandbox version of this request trait
