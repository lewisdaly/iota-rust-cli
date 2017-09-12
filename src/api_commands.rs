#[allow(dead_code)]

use serde_json;

pub trait Command {
    //serialize the command for use in post request
    fn serialize(&self) -> String;
}

#[derive(Debug)]
#[derive(Serialize)]
pub struct CommandGetBalance {
    command: String,
    addresses: [String; 1],
    threshold: i32,
}

#[derive(Serialize)]
pub struct CommandGetNodeInfo {
    command: String,
}

#[derive(Serialize)]
pub struct CommandGetInclusionStates {
    command: String,
    transactions: [String; 1],
    tips: [String; 1],
}

#[derive(Serialize)]
pub struct CommandGetTrytes {
    command: String,
    hashes: [String; 1],
}

impl Command for CommandGetBalance {
    fn serialize(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }
}

impl Command for CommandGetNodeInfo {
    fn serialize(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }
}

impl Command for CommandGetInclusionStates {
    fn serialize(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }
}

impl Command for CommandGetTrytes {
    fn serialize(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }
}

pub fn get_balance(address: String, threshold: i32) -> Option<Box<Command>> {

    //In the future, we could support multiple addresses. Just 1 for now.
    let addresses = [address];

    let command: CommandGetBalance = CommandGetBalance {command: "getBalances".to_owned(), addresses: addresses, threshold: threshold};
    Some(Box::new(command))
}


pub fn get_node_info() -> Option<Box<Command>> {
    let command: CommandGetNodeInfo = CommandGetNodeInfo { command: "getNodeInfo".to_owned() };

    Some(Box::new(command))
}


pub fn get_inclusion_states(transaction: String, tip: String) -> Option<Box<Command>> {
    let transactions = [transaction];
    let tips = [tip];
    let command: CommandGetInclusionStates = CommandGetInclusionStates {
        command: "getInclusionStates".to_owned(),
        transactions: transactions,
        tips: tips
    };

    Some(Box::new(command))
}

pub fn get_trytes(hash: String) -> Option<Box<Command>> {
    let hashes = [hash];
    let command: CommandGetTrytes = CommandGetTrytes { command: "getTrytes".to_owned(), hashes: hashes};

    Some(Box::new(command))
}
