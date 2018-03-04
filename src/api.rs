use api_commands;
use api_models;
use request::IotaRequest;
use models::*;
use alloc::Vec;
use trytes::string::char_to_trits;
use trytes::Trit;
use serde_json;
use curl::*;
use curl_cpu::CpuCurl;
use sign::iss::KEY_LENGTH;
use sign::iss::DIGEST_LENGTH;
use sign::iss::ADDRESS_LENGTH;
use sign::iss::SIGNATURE_LENGTH;
use sign::iss::subseed;
use sign::iss::key;
use sign::iss::digest_key;
use sign::iss::address;




/**
 * Ref: https://github.com/iotaledger/iota.lib.js#getnewaddress
 */
pub fn generate_address(request: &IotaRequest, seed: &str, index: Option<i32>, security: i32) {
    //TODO: remove
    println!("Generating address! {:?}", seed);

    //TODO: validate seed


    //Generate an address.
    //If no index is supplied, we generate an address, and call findTransactions on that address.
    //If no transactions exist, return, else increment the index and try again

    //TODO: remove, just for testing now.
    let seed: Vec<Trit> = seed
        .chars()
        .flat_map(char_to_trits)
        .cloned()
        .collect();
    let address = new_address(seed, 1, 1);
    //TODO: convert address to string properly
    println!("Address: {:?}", address);
}

/**
 * Generate a new address based on the index.
 */
fn new_address(seed: Vec<Trit>, index: isize, security: usize) -> Vec<Trit> {

    //From iss.rs, line 297. Not sure if this will even work
    let mut c1 = CpuCurl::<Trit>::default();
    let mut c2 = CpuCurl::<Trit>::default();
    let mut key_space = vec![0; KEY_LENGTH];
    let mut digest_space = vec![0; DIGEST_LENGTH];
    let mut address_space = vec![0; ADDRESS_LENGTH];

    subseed::<CpuCurl<Trit>>(&seed, index, &mut key_space, &mut c1);
    c1.reset();
    key(&mut key_space, security, &mut c1);
    c1.reset();
    digest_key::<Trit, CpuCurl<Trit>>(&key_space, &mut digest_space, &mut c1, &mut c2);
    c1.reset();
    address::<Trit, CpuCurl<Trit>>(&mut digest_space, &mut c1);
    address_space.clone_from_slice(&digest_space);

    address_space
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

    //Assume just 1 for now
    let mut trytes  = response["trytes"][0].to_string();
    //TODO: Deserialize properly. This will do for now
    trytes = str::replace(trytes.as_str(), "\"", "");
    trytes = str::replace(trytes.as_str(), "\\", "");

    let tx: Vec<Trit> = trytes.as_str().chars().flat_map(char_to_trits).cloned().collect();

    //Get a TransactionView struct from the trits
    let txview = TransactionView::from_trits(tx.as_slice()).unwrap();
    let tx_model = api_models::TransactionModel::from_transaction_view(&txview).unwrap();
    println!("{:?}", serde_json::to_string(&tx_model).unwrap())
}
