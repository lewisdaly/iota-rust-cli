use api_commands;
use request::IotaRequest;

/**
 * Ref: https://github.com/iotaledger/iota.lib.js#getnewaddress
 */
pub fn generate_address() {
    println!("Generating address!");

    //TODO: validate seed

}

pub fn balance(request: &IotaRequest, address: &str) {
    println!("Checking balance");

    //TODO: check we have a valid seed

    let command = api_commands::get_balance(address.to_owned(), 1);
    let response = request.make_request(command.unwrap());
    println!("Response is: {:?}", response.unwrap());

    //Send the request to the api
}
