
extern crate serde_json;

extern crate iota_api;
use iota_api::request::IotaClient;
use iota_api::api;
use iota_api::api_commands::FindTransactionsType;

#[test]
fn live_find_transactions() {
    // let address = "XFAELMMFKBKNGAYLKNFOLHMOTAM9IVOLABDLNKNWUIFK9QJXEJKGAUHMEPMYPGLPFBWZAXJMFZYCWMQEA";
    let tag = "MY9CUSTOM9TAG99999999999999";

    let client = &IotaClient { protocol: "http://".to_owned(), host: "node.iota.com.tw".to_owned(), port: 5000 };
    let result = match api::find_transactions(client, FindTransactionsType::Tags, tag.to_owned()) {
        Ok(n) => n,
        Err(err) => panic!("called `Result::unwrap()` on an `Err` value: {:?}", err),
    };

    let json = serde_json::to_string_pretty(&result).unwrap();
    println!("json is: {:?}", json);
}
