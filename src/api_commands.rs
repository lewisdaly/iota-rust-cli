
#[derive(Debug)]
//TODO: this struct can implement a serialize trait!
pub struct CommandGetBalance {
    command: String,
    addresses: [String; 1],
    threshold: i32,
}

pub trait Command {
    //But how do we make this generic?
    // fn new([String; 1], i32) -> Self;

    //serialize the command for use in post request
    fn serialize(&self) -> &str;
}

impl Command for CommandGetBalance {
    // fn new(addresses: [String; 1], threshold: i32) -> CommandGetBalance {
    //     CommandGetBalance { command: "getBalance".to_owned(), addresses: addresses, threshold:threshold}
    // }

    fn serialize(&self) -> &str {
        "This is the serialized command"
    }
}

pub fn get_balance(address: String, threshold: i32) -> Option<Box<Command>> {

    //In the future, we could support multiple addresses. Just 1 for now.
    let addresses = [address];

    let command: CommandGetBalance = CommandGetBalance {command: "getBalance".to_owned(), addresses: addresses, threshold: threshold};
    Some(Box::new(command))
    // CommandGetBalance { command: "getBalance".to_owned(), addresses: addresses, threshold:threshold }
}
