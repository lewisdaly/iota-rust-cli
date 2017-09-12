#[allow(dead_code)]

pub trait Command {
    //serialize the command for use in post request
    fn serialize(&self) -> String;
}

#[derive(Debug)]
pub struct CommandGetBalance {
    command: String,
    addresses: [String; 1],
    threshold: i32,
}

pub struct CommandGetNodeInfo {
    command: String,
}

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
    //TODO: don't serialize ourselves - use this: #[derive(Serialize, Deserialize)]
    fn serialize(&self) -> String {
        let json = json!({
            "command": self.command,
            "addresses": self.addresses,
            "threshold": self.threshold
        });
        json.to_string()
    }
}

impl Command for CommandGetNodeInfo {
    fn serialize(&self) -> String {
        let json = json!({
            "command": self.command,
        });

        json.to_string()
    }
}

impl Command for CommandGetInclusionStates {
    fn serialize(&self) -> String {
        let json = json!({
            "command": self.command,
            "transactions": self.transactions,
            "tips": self.tips,
        });
        json.to_string()
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
