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

    let command = api_commands::get_balance(address.to_owned(), 100);
    let response = request.make_request(command.unwrap());
    println!("{:?}", response.unwrap().to_string());
}


pub fn tx_status(request: &IotaRequest, tx_hash: &str) {

    let node_command = api_commands::get_node_info();
    let node_response = request.make_request(node_command.unwrap()).unwrap();

    //TODO: we should probably make sure this response is the correct format
    //TODO: get the most recent solid milestone from node_response
    let mut tip = node_response["latestSolidSubtangleMilestone"].to_string();

    //TODO: Deserialize properly. This will do for now
    tip = str::replace(tip.as_str(), "\"", "");
    tip = str::replace(tip.as_str(), "\\", "");

    println!("{:?}", tip);

    let inclusion_command = api_commands::get_inclusion_states(tx_hash.to_owned(), tip.to_owned());
    let inclusion_response = request.make_request(inclusion_command.unwrap());

    println!("{:?}", inclusion_response.unwrap());
}


pub fn tx_show(request: &IotaRequest, tx_hash: &str) {
    //First, perform getTrytes
    let command = api_commands::get_trytes(tx_hash.to_owned());
    let response = request.make_request(command.unwrap()).unwrap();

    let trytes  = response["trytes"].to_string();

    println!("{:?}", trytes);


    //Then do rust lib magic

}
