extern crate iota_api;
use iota_api::request::IotaClient;
use iota_api::api;
use iota_api::api_commands::FindTransactionsType;

#[test]
fn live_find_transactions() {
    let address = "XFAELMMFKBKNGAYLKNFOLHMOTAM9IVOLABDLNKNWUIFK9QJXEJKGAUHMEPMYPGLPFBWZAXJMFZYCWMQEA";

    let client = &IotaClient { protocol: "http://".to_owned(), host: "node.iota.com.tw".to_owned(), port: 5000 };
    match api::find_transactions(client, FindTransactionsType::Addresses, address.to_owned()) {
        Ok(n) => println!("result: {:?}", n),
        Err(err) => panic!("called `Result::unwrap()` on an `Err` value: {:?}", err),
    };

}
