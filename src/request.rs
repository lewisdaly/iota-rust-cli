/**
 * Make the api request
 */

extern crate serde_json;
extern crate futures;
extern crate hyper;
extern crate tokio_core;

use std::io::{self};
use self::futures::{Future, Stream};
use self::hyper::Client;
use self::hyper::Error;
use self::tokio_core::reactor::Core;
use self::serde_json::Value;

use api_commands::Command;

use utils::format_url;

//TODO: make a sandbox client if need be, and implement the Request trait!
pub struct IotaClient {
    pub host: &'static str,
    pub port: i32
}

pub trait Request {
    //This will also have to take a generic command
    fn make_request(&self, Box<Command>) -> Result<Value, Error>;
}

impl Request for IotaClient {
    // //Setup the api client
    // fn new() -> IotaClient {
    //     let mut core = Core::new()?;
    //     let client = Client::new(&core.handle());
    //     //TODO: save this client to the internal IotaClient struct...
    //     IotaClient { host: "https://google.com", port: 12345 }
    // }

    //Implement this request
    fn make_request(&self, command: Box<Command>) -> Result<Value, Error> {
        //TODO: sign the request!

        //TODO: we should be reusing this client object - or at core!
        let mut core = Core::new().unwrap();
        let client = Client::new(&core.handle());

        println!("Making request! {}", command.serialize());
        let uri = "http://httpbin.org/ip".parse().unwrap();
        let work = client.get(uri).and_then(|res| {
            println!("Response: {}", res.status());

            res.body().concat2().and_then(move |body| {
                let v: Value = serde_json::from_slice(&body).map_err(|e| {
                    io::Error::new(
                        io::ErrorKind::Other,
                        e
                    )
                })?;
                println!("current IP address is {}", v["origin"]);
                Ok(v)
            })
        });

        core.run(work)
    }
}


//TODO: we can also implement a sandbox version of this request trait