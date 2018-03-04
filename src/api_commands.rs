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

//findTransactions takes an array of budnels, addresses, tags or approvees
pub enum FindTransactionsType {
    Bundle,
    Addresses,
    Tags,
    Approvees
}

#[derive(Serialize)]
//ref: https://iota.readme.io/reference#findtransactions
pub struct CommandFindTransactions {
    command: String,
    bundles: Option<[String; 1]>,   //optional, just 1 required
    addresses: Option<[String; 1]>, //optional, just 1 required
    tags: Option<[String; 1]>,      //optional, just 1 required
    approvees: Option<[String; 1]>, //optional, just 1 required

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


impl Command for CommandFindTransactions {
    fn serialize(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }
}


pub fn get_balance(address: String, threshold: i32) -> Option<Box<Command>> {

    //In the future, we could support multiple addresses. Just 1 for now.
    let addresses = [address];

    //TODO: implement default for structs
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


pub fn find_transactions(transaction_type: FindTransactionsType, param: String) -> Option<Box<Command>> {

    let command: CommandFindTransactions = CommandFindTransactions {
        command: "findTransactions".to_owned(),
        //TODO: There's probably an easier way...
        bundles: match transaction_type {
            FindTransactionsType::Bundle => Some([param.to_owned()]),
            _ => None
        },
        addresses: match transaction_type {
            FindTransactionsType::Addresses => Some([param.to_owned()]),
            _ => None
        },
        tags: match transaction_type {
            FindTransactionsType::Tags => Some([param.to_owned()]),
            _ => None
        },
        approvees: match transaction_type {
            FindTransactionsType::Approvees => Some([param.to_owned()]),
            _ => None
        }
    };

    return Some(Box::new(command));
}
