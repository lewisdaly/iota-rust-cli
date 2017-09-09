use api_commands;
use api_commands::CommandGetBalance;
use api_commands::Command;
use request::Request;

pub extern crate futures;
pub extern crate hyper;
pub extern crate tokio_core;

/**
 * Ref: https://github.com/iotaledger/iota.lib.js#getnewaddress
 */
pub fn generate_address() {
    println!("Generating address!");

    //TODO: validate seed

}

pub fn balance(request: &Request, seed: &str) {
    println!("Checking balance");

    //TODO: check we have a valid seed

    let command = api_commands::get_balance(seed.to_owned(), 1);
    request.make_request(command.unwrap());

    //Send the request to the api
}
