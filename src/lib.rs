mod boletin;
mod articles;

pub struct Config {
    addresses: Vec<String>,
    send_email: bool,
    stdout: bool,
    search_string: String, // check if str with lifetime parameter can work
    date: String
}

impl Config {
    pub fn new(address_vec: Vec<&str>, send_email: bool, stdout: bool, search_string: &str, date: &str) -> Config {

        let addresses = address_vec.iter().map(|&a| String::from(a)).collect();

        Config {
            addresses,
            send_email,
            stdout,
            search_string: String::from(search_string),
            date: String::from(date),
        }
    }
}

fn process_articles(config: &Config) -> Option<Vec<articles::Article>> {
    // TODO: Enable from-to dates
    let res = boletin::fetch_articles(&config.search_string, &config.date, &config.date);

    match res {
        Ok(articles) => Some(articles),
        Err(e) => {
            eprintln!("Error was: {}", e);
            None
        }
    }
}

pub fn run(config: Config) -> () {
    let articles = process_articles(&config).unwrap_or(vec![]);

    if config.send_email { send_email(&config.addresses, &articles) }

    if config.stdout { print_articles(&articles) }
}

// TODO: Implement
fn send_email(_addresses: &Vec<String>, _articles: &Vec<articles::Article>) {
    println!("Email was sent!");
}

fn print_articles(articles: &Vec<articles::Article>) {
    for (i, a) in articles.iter().enumerate() {
        println!("Article #{}: {}", i, a);
    }
}
