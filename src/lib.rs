mod articles;
mod boletin;
mod mailer;

pub struct Config<'a> {
    addresses: Vec<String>,
    send_email: bool,
    stdout: bool,
    search_string: &'a str,
    date: String,
}

impl<'a> Config<'a> {
    pub fn new(
        address_vec: Vec<&str>,
        send_email: bool,
        stdout: bool,
        search_string: &'a str,
        date: &str,
    ) -> Config<'a> {
        let addresses = address_vec.iter().map(|&a| String::from(a)).collect();

        Config {
            addresses,
            send_email,
            stdout,
            search_string: search_string,
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
    let mut articles: Vec<articles::Article> = vec![];

    if config.send_email || config.stdout {
        articles = process_articles(&config).unwrap_or(vec![]);
    }

    if config.stdout {
        print_articles(&articles)
    }

    if config.send_email {
        send_emails(config, &articles)
    }
}

fn send_emails(config: Config, articles: &Vec<articles::Article>) {
    config
        .addresses
        .iter()
        .for_each(|a| mailer::send_email(a, articles, &config.date.to_string()))
}

fn print_articles(articles: &Vec<articles::Article>) {
    for (i, a) in articles.iter().enumerate() {
        println!("Article #{}: {}", i, a);
    }
}
