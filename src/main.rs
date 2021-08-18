use bofe::{Config};
use clap::{load_yaml, App};
use chrono::Utc;

// TODO:
// Extract args to struct
// function recieves args and handles behaviour
fn main() {
    let yaml = load_yaml!("cli.yml");
    let m = App::from(yaml).get_matches();
    let search_string: &str;
    let mut from_date: &str = &Utc::today().naive_utc().format("%Y-%m-%d").to_string();

    if m.is_present("date") {
        from_date = m.value_of("date").unwrap();
    }

    if m.is_present("search_string") {
        search_string = m.value_of("search_string").unwrap();
    } else {
        search_string = "Policia Seguridad Aeroportuaria";
    }

    let addresses = vec!["test@email.com"];

    let config = Config::new(addresses, m.is_present("email"), m.is_present("verbose"), search_string, from_date);

    bofe::run(config);
}
