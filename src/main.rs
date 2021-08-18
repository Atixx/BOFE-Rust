use bofe::boletin;
use chrono::Utc;

// TODO:
// Extract args to struct
// function recieves args and handles behaviour
fn main() {
    use clap::{load_yaml, App};

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

    // TODO: Enable from-to dates
    let res = boletin::fetch_articles(search_string, from_date, from_date);
    // let res: Result<(), Box<dyn std::error::Error>> = Ok(());

    match res {
        Ok(_) => (),
        Err(e) => eprintln!("Error was: {}", e)
    }

    if m.is_present("email") {
        // TODO:
        // Send to default address
        // send to input addresses
        // println!("{}", build_query());
        println!("Email was sent");
    }

    if m.is_present("verbose") {
        println!("Articles found are: \n{}", "articles");
    }
}
