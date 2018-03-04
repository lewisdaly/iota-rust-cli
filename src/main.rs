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


use clap::App;

mod api;
mod api_commands;
mod api_models;
mod request;
mod utils;
pub mod enums;
pub use enums::TestEnum;


use request::IotaClient;


fn main() {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    if let Some(config) = matches.value_of("config") {
        println!("WARNING: specifying the --config argument is not yet implemented. This argument will be ignored.");
    }

    let host = matches.value_of("host").unwrap();
    let port: i32 = matches.value_of("port").unwrap().parse().unwrap();

    //TODO: validate host and port
    let client = &IotaClient { protocol: "http://".to_owned(), host: host.to_owned(), port: port };

    match matches.subcommand() {
        ("generate-address", Some(matches)) => {
            let index = match matches.value_of("index") {
                None => None,
                Some(index) => {
                    let index_int: i32 = FromStr::from_str(index).unwrap();
                    Some(index_int)
                }
            };
            let security: i32 = FromStr::from_str(matches.value_of("security").unwrap()).unwrap();

            match matches.value_of("seed") {
                None => {
                    let seed = rpassword::prompt_password_stdout("Enter Seed (will be hidden): ").unwrap();
                    api::generate_address(client, &seed, index, security);
                }
                Some(seed) => api::generate_address(client, seed, index, security),
            }
        },
        ("balance", Some(matches)) => {
            let address = matches.value_of("address").unwrap();
            api::balance(client, address);
        },
        ("tx-status", Some(matches)) => {
            let tx_hash = matches.value_of("tx_hash").unwrap();
            api::tx_status(client, tx_hash);
        },
        ("tx-show", Some(matches)) => {
            let tx_hash = matches.value_of("tx_hash").unwrap();
            api::tx_show(client, tx_hash);
        },
        _ => println!("Other command was used")
    }
}
