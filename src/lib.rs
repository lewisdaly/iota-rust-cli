#![feature(alloc)]
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_extern_crates)]

use std::io::{stdin,stdout,Write};
use std::str::FromStr;


#[macro_use]
extern crate clap;

#[macro_use]
extern crate serde_json;

#[macro_use]
extern crate serde_derive;

extern crate serde;
extern crate alloc;
extern crate rpassword;

extern crate iota_models as models;
extern crate iota_trytes as trytes;
extern crate iota_sign as sign;
//Not sure what the difference is here.
extern crate iota_curl as curl;
extern crate iota_curl_cpu as curl_cpu;


pub use clap::App;

pub mod api;
pub mod api_commands;
pub mod api_models;
pub mod request;
pub mod utils;

pub use request::IotaClient;
