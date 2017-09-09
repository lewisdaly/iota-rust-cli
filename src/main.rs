#[macro_use]
extern crate clap;

use clap::App;

mod api;
mod api_commands;
mod request;

use request::Client;

fn main() {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    //TODO: make a client or something!
    let client = &Client { host: "google.com", port: 12345 };

    let value = matches.value_of("balance");
    println!("{:?}", value);

    // match matches.subcommand_name() {
    //     Some("generate-address") => api::generate_address(),
    //     Some("balance") => api::balance(),
    //     _  => println!("Other command was used")
    // }

    match matches.subcommand() {
        ("generate-address", Some(matches)) => {
            println!("Generating Address: {}", matches.value_of("seed").unwrap())
        },
        ("balance", Some(matches)) => {
            println!("Checking balance: {}", matches.value_of("seed").unwrap());
            //TODO: we can prompt user for seed here if we like!
            let seed = matches.value_of("seed").unwrap();
            api::balance(client, seed);
        },
        _ => println!("Other command was used")
    }

}
