use soup::prelude::*;
use serde::Deserialize;
use std::collections::HashMap;
use reqwest::header::{HeaderMap, HeaderValue, USER_AGENT, CONTENT_TYPE, ACCEPT_ENCODING, ACCEPT, CONNECTION};
use crate::articles::Article;
use chrono::naive::NaiveDate;


mod query;

const POST_URL: &str = "https://www.boletinoficial.gob.ar/busquedaAvanzada/realizarBusqueda";
const BASE_URL: &str = "https://www.boletinoficial.gob.ar";

#[derive(Deserialize, Debug)]
struct BoletinResponse {
    error: u32,
    content: BoletinContent,
    mensajes: Vec<String>
}

#[derive(Deserialize, Debug)]
struct BoletinContent {
    html: String,
    sig_pag: u32,
    ult_seccion: String,
    ult_rubro: String,
    cantidad_result_seccion: HashMap<u32, u32>,
}


fn request_articles(query_info: &query::QueryInfo) -> Result<BoletinResponse, Box<dyn std::error::Error>> {
    let client = reqwest::blocking::Client::new();


    fn construct_headers() -> HeaderMap {
        let mut headers = HeaderMap::new();
        headers.insert(USER_AGENT, HeaderValue::from_static("Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_5) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/84.0.4147.89 Safari/537.36"));
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/x-www-form-urlencoded; charset=UTF-8"));
        headers.insert(ACCEPT_ENCODING, HeaderValue::from_static("gzip, deflate"));
        headers.insert(ACCEPT, HeaderValue::from_static("*/*"));
        headers.insert(CONNECTION, HeaderValue::from_static("keep-alive"));

        headers
    }

    let query = query::BoletinQuery::new(query_info).build_query()?;

    let res = client.post(POST_URL)
        .headers(construct_headers())
        .body(query)
        .send()?;


    let body = res.json::<BoletinResponse>()?;

    Ok(body)
}

pub fn fetch_articles(search_string: &str, from_date: &str, to_date: &str ) -> Result<Vec<Article>, Box<dyn std::error::Error>> {

    let from = NaiveDate::parse_from_str(&from_date, "%Y-%m-%d")?;
    let to = NaiveDate::parse_from_str(&to_date, "%Y-%m-%d")?;

    let query_info = query::QueryInfo::new(
        &search_string,
        from,
        to
    );

    let body = request_articles(&query_info)?; // TODO: exit early if errors
    let content = &body.content;

    let soup = Soup::new(&content.html);
    let articles = extract_articles(&soup);

    Ok(articles)
}

// TODO: Add tests and refactor is_link away
fn extract_articles(soup: &Soup) -> Vec<Article> {
    let mut articles: Vec<Article> = vec![];

    for article in soup.tag("p").class("item").find_all() {

        let mut parents = article.parents();
        let anchor = parents.find(|tag| is_link(&tag.name().to_string() ));
        let mut link: String = String::from(BASE_URL);

        // TODO: use unwrap
        if let Some(a) = anchor {
            let l = a.get("href");
            if let Some(href) = l {
                link.push_str(&href);
            }
        }

        let raw_title = String::from(&article.text());
        let title = raw_title.trim().replace('\u{a0}', " ");

        articles.push(Article { title, link });
    }

    articles
}

fn is_link(tag: &str) -> bool {
            tag == "a"
}
