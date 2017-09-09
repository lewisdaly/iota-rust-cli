
#[derive(Debug)]
//TODO: this struct can implement a serialize trait!
pub struct CommandGetBalance<'a> {
    command: &'a str,
    addresses: [&'a str; 1],
    threshold: i32,
}

pub trait Command {
    //serialize the command for use in post request
    fn serialize(&self) -> &str;
}

impl Command for CommandGetBalance<'a> {
    fn serialize(&self) -> &str {
        "This is the serialized command"
    }
}


pub fn get_balance<'a>(address: &'a str, threshold: i32) -> CommandGetBalance<'a> {

    //In the future, we could support multiple addresses. Just 1 for now.
    let addresses = [address];
    CommandGetBalance { command: "getBalance", addresses: addresses, threshold:threshold}
}
