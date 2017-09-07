#[macro_use]
extern crate clap;

use clap::App;

mod generate_address;

fn main() {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    match matches.subcommand_name() {
        Some("generate-address") => generate_address::generate_address(),
        _  => println!("Other command was used")
    }

}
