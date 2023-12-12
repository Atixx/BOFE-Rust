extern crate dotenv;

use bofe::Config;
use chrono::Utc;
use clap::{load_yaml, App};
use dotenv::dotenv;

// High Level TODO:
// get default address + override with argument address
// need to handle address format in argument
// Move all todos to readme
fn main() {
    dotenv().ok();
    let yaml = load_yaml!("cli.yml");
    let m = App::from(yaml).get_matches();
    // let search_string: &str;
    let mut from_date: &str = &Utc::today().naive_utc().format("%Y-%m-%d").to_string();

    if m.is_present("date") {
        from_date = m.value_of("date").unwrap();
    }

    let search_string =
        if m.is_present("search_string") {
            m.value_of("search_string").unwrap()
        } else {
            "Policia Seguridad Aeroportuaria"
        };

    // TODO: Figure out addresses
    if m.is_present("address") {
        println!("{}", m.value_of("address").unwrap())
    };

    let addresses = vec!["Cargo Testing <peteg73374@ampswipe.com>"];
    // let addresses = vec!["test@email.com", "another@address.com"];

    let config = Config::new(
        addresses,
        m.is_present("email"),
        m.is_present("verbose"),
        search_string,
        from_date,
    );

    bofe::run(config);
}
