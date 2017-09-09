/**
 * Make the api request
 */

use std::io::{self, Write};
use api::futures::{Future, Stream};
use api::hyper::Client;
use api::hyper::Uri;
use api::tokio_core::reactor::Core;

use api_commands::Command;

//TODO: make a sandbox client if need be, and implement the Request trait!
pub struct IotaClient {
    pub host: &'static str,
    pub port: i32
}

pub trait Request {
    //This will also have to take a generic command
    fn make_request(&self, Box<Command>) -> bool;
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
    fn make_request(&self, command: Box<Command>) -> bool {
        //TODO: sign the request!

        //TODO: we should be reusing this client object - or at core!
        let mut core = Core::new().unwrap();
        let client = Client::new(&core.handle());

        println!("Making request! {}", command.serialize());
        // let uri = Uri::parse(self.host).unpack();
        // let url = self.host.parse::<Uri>().unwrap();
        let uri = "http://httpbin.org/ip".parse().unwrap();

        // let work = client.get(uri).map(|res| {
        //     println!("Response: {}", res.status());
        //     println!("Headers: \n{}", res.headers());
        // });

        let work = client.get(uri).and_then(|res| {
            println!("Response: {}", res.status());

            res.body().for_each(|chunk| {
                io::stdout()
                    .write_all(&chunk)
                    .map(|_| ())
                    .map_err(From::from)
            })
        });

        core.run(work).unwrap();

        false
    }
}


//TODO: we can also implement a sandbox version of this request trait
