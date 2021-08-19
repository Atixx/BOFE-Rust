use super::articles::Article;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};
use std::env;

struct OutgoingConfig<'a> {
    from_address: &'a str,
    reply_to: &'a str,
    subject_base: &'a str,
    smtp_server: &'a str,
}

fn resolve_smtp() -> (String, String) {
    let user = env::var("SMTP_USERNAME").unwrap();
    let pass = env::var("SMTP_PASSWORD").unwrap();
    (user, pass)
}

const CREDS: OutgoingConfig = OutgoingConfig {
    from_address: "BoletinBot <boletinoficialnews@gmail.com>",
    reply_to: "NoReply <no_reply_bofe@bofe.no>",
    subject_base: "Novedades Boletin Oficial: ",
    smtp_server: "smtp.gmail.com",
};

fn build_email(articles: &Vec<Article>, to_address: &str, subject: &str) -> Message {
    let mut body: String = String::from("Articles found for:\n");

    for article in articles {
        let mut content = article.to_string();
        content.push_str(&"\n");
        body.push_str(&content);
    }

    Message::builder()
        .from(CREDS.from_address.parse().unwrap())
        .reply_to(CREDS.reply_to.parse().unwrap())
        .to(to_address.parse().unwrap())
        .subject(subject)
        .body(body)
        .unwrap()
}

pub fn send_email(address: &str, articles: &Vec<Article>, date: &str) -> () {
    // TODO: Handle to-from dates in subject
    let mut subject = String::from(CREDS.subject_base);
    subject.push_str(date);
    let email_message = build_email(articles, address, &subject);

    let creds = Credentials::new(resolve_smtp().0, resolve_smtp().1);

    let mailer = SmtpTransport::relay(CREDS.smtp_server)
        .unwrap()
        .credentials(creds)
        .build();

    // TODO: implement
    match mailer.send(&email_message) {
        Ok(_) => println!("Email sent successfully!"),
        Err(e) => panic!("Could not send email: {:?}", e),
    }
}
