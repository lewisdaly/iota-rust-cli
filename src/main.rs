#![feature(alloc)]
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_extern_crates)]


#[macro_use]
extern crate clap;

#[macro_use]
extern crate serde_json;

#[macro_use]
extern crate serde_derive;

extern crate serde;
extern crate iota_models as models;
extern crate iota_trytes as trytes;
extern crate alloc;

// pub use trytes::trits_to_string;

use clap::App;

mod api;
mod api_commands;
mod api_models;
mod request;
mod utils;

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
    let client = &IotaClient { host: host.to_owned(), port: port };

    match matches.subcommand() {
        ("generate-address", Some(matches)) => {
            println!("Generating Address: {}", matches.value_of("seed").unwrap())
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
