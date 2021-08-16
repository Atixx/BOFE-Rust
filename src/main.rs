use bofe::boletin;

fn main() {
    use clap::{load_yaml, App};

    let yaml = load_yaml!("cli.yml");
    let m = App::from(yaml).get_matches();

    let res = boletin::fetch_results();
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
