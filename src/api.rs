use api_commands;
use request::Request;

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

    let command = api_commands::get_balance(seed, 1);
    println!("{:?}", command);

    //Send the request to the api
}
