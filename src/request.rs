/**
 * Make the api request
 */

extern crate serde_json;
extern crate futures;
extern crate hyper;
extern crate tokio_core;
extern crate reqwest;

use std::io::{self};
use self::futures::{future, Future, Stream, done};
use self::hyper::Client;
use self::hyper::Error;
use self::tokio_core::reactor::Core;
use self::serde_json::Value;

use self::hyper::{Method, Request};
use self::hyper::header::{ContentLength, ContentType};
// use self::reqwest::header::{Headers, UserAgent, ContentType};

use api_commands::Command;
use utils::format_url;

//TODO: make a sandbox client if need be, and implement the Request trait!
pub struct IotaClient {
    pub protocol: String,
    pub host: String,
    pub port: i32
}

pub trait IotaRequest {
    //This will also have to take a generic command
    fn make_request(&self, Box<Command>) -> Result<Value, Error>;
    // fn new_make_request(&self, Box<Command>) -> Result<(), reqwest::Error>;

}

impl IotaRequest for IotaClient {
    // //Setup the api client
    // fn new() -> IotaClient {
    //     let mut core = Core::new()?;
    //     let client = Client::new(&core.handle());
    //     //TODO: save this client to the internal IotaClient struct...
    //     IotaClient { host: "https://google.com", port: 12345 }
    // }

    // fn new_make_request(&self, command: Box<Command>) -> Result<(), reqwest::Error> {
    //     let client = reqwest::Client::new();
    //     let uri_string = format_url(self.protocol.to_owned(), self.host.to_owned(), self.port, "".to_owned());
    //
    //     let json = command.serialize();
    //     println!("{:?}", json);
    //
    //     let res = match client.post(uri_string.parse().unwrap()).json(&json).send() {
    //         Ok(r) => { println!("{:?}", r);},
    //         Err(e) => {
    //             println!("{:?}", e);
    //         }
    //     };
    //
    //     return Ok(());
    // }

    //Implement this request
    fn make_request(&self, command: Box<Command>) -> Result<Value, Error> {
        //TODO: sign the request!

        //TODO: we should be reusing this client object - or at core!
        let mut core = Core::new().unwrap();
        let client = Client::new(&core.handle());

        let uri_string = format_url(self.protocol.to_owned(), self.host.to_owned(), self.port, "".to_owned());
        let uri = uri_string.parse().unwrap();

        let json = command.serialize();
        println!("{:?}", json);
        let mut req = Request::new(Method::Post, uri);
        req.headers_mut().set(ContentType::json());
        req.headers_mut().set(ContentLength(json.len() as u64));
        req.set_body(json);

        let res = client.post("http://example.domain")
            .body("foo=bar")
            .send()
            .unwrap();

        // let work = client.request(req)
        //     .and_then(|res| {
        //         println!("Response: {}", res.status());
        //         println!("Headers: \n{}", res.headers());
        //
        //         res.body().fold(Vec::new(), |mut v, chunk| {
        //             v.extend(&chunk[..]);
        //             future::ok<_, Error>(v)
        //         }).and_then(|chunks| {
        //             let v: Value = serde_json::from_slice(&chunks).map_err(|e| {
        //                 io::Error::new(
        //                     io::ErrorKind::Other,
        //                     e
        //                 )
        //             })?;
        //             Ok(v)
        //         })
        //     });

        // let work = client.request(req)
        //     .and_then(|res| {
        //         if !res.status().is_success() {
        //             println!("Request failed with status{:?}", res.status());
        //             return future::ok::<_, Error>(res.status());
        //         }
        //
        //         res.body().concat2()
        //     })
        //     .and_then(move |body| {
        //         let v: Value = serde_json::from_slice(&body).map_err(|e| {
        //             io::Error::new(
        //                 io::ErrorKind::Other,
        //                 e
        //             )
        //         })?;
        //         Ok(v)
        //     });

        // let work = client.request(req).map(|res| {
        //     match res.status().is_success() {
        //         false => {
        //             let err = io::Error::new(io::ErrorKind::Other, "error");
        //             Ok(());
        //         },
        //         true => {
        //             res.body().concat2().and_then(move |body| {
        //                 let v: Value = serde_json::from_slice(&body).map_err(|e| {
        //                     io::Error::new(
        //                         io::ErrorKind::Other,
        //                         e
        //                     )
        //                 })?;
        //                 Ok(v)
        //             })
        //         }
        //     }
        // });

        // core.run(work)
    }
}


//TODO: we can also implement a sandbox version of this request trait
