extern crate serde_json;

use api_commands;
use request::IotaRequest;
use self::serde_json::Value;
use self::serde_json::ser;

/**
 * Ref: https://github.com/iotaledger/iota.lib.js#getnewaddress
 */
pub fn generate_address() {
    println!("Generating address!");

    //TODO: validate seed

}

pub fn balance(request: &IotaRequest, address: &str) {

    //TODO: check we have a valid address

    let command = api_commands::get_balance(address.to_owned(), 1);
    let response = request.make_request(command.unwrap());
    println!("{:?}", response.unwrap().to_string());
}
