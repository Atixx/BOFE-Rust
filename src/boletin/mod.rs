use crate::articles::Article;
use chrono::naive::NaiveDate;
use reqwest::header::{
    HeaderMap, HeaderValue, ACCEPT, ACCEPT_ENCODING, CONNECTION, CONTENT_TYPE, USER_AGENT,
};
use serde::Deserialize;
use soup::prelude::*;
use std::collections::HashMap;

mod query;

const POST_URL: &str = "https://www.boletinoficial.gob.ar/busquedaAvanzada/realizarBusqueda";
const BASE_URL: &str = "https://www.boletinoficial.gob.ar";

#[derive(Deserialize, Debug)]
struct BoletinResponse {
    error: u32,
    content: BoletinContent,
    mensajes: Vec<String>,
}

#[derive(Deserialize, Debug)]
struct BoletinContent {
    html: String,
    sig_pag: u32,
    ult_seccion: String,
    ult_rubro: String,
    cantidad_result_seccion: ResultsBySection,
}

#[derive(Deserialize, Debug)]
#[serde(untagged)]
enum ResultsBySection {
    ValueResponse(HashMap<String, u32>),
    Empty(Vec<u32>),
}

fn request_articles(
    query_info: &query::QueryInfo,
) -> Result<BoletinResponse, Box<dyn std::error::Error>> {
    let client = reqwest::blocking::Client::new();

    fn construct_headers() -> HeaderMap {
        let mut headers = HeaderMap::new();
        headers.insert(USER_AGENT, HeaderValue::from_static("Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_5) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/84.0.4147.89 Safari/537.36"));
        headers.insert(
            CONTENT_TYPE,
            HeaderValue::from_static("application/x-www-form-urlencoded; charset=UTF-8"),
        );
        headers.insert(ACCEPT_ENCODING, HeaderValue::from_static("gzip, deflate"));
        headers.insert(ACCEPT, HeaderValue::from_static("*/*"));
        headers.insert(CONNECTION, HeaderValue::from_static("keep-alive"));

        headers
    }

    let query = query::BoletinQuery::new(query_info).build_query()?;

    let res = client
        .post(POST_URL)
        .headers(construct_headers())
        .body(query)
        .send()?;

    let body = res.json::<BoletinResponse>()?;

    Ok(body)
}

/// Query and parse results of boletin oficial's to a list of articles
/// pagination is not built in so it will return at most 100 results
pub fn fetch_articles(
    search_string: &str,
    from_date: &str,
    to_date: &str,
) -> Result<Vec<Article>, Box<dyn std::error::Error>> {
    let from = NaiveDate::parse_from_str(&from_date, "%Y-%m-%d")?;
    let to = NaiveDate::parse_from_str(&to_date, "%Y-%m-%d")?;

    let query_info = query::QueryInfo::new(&search_string, from, to);

    let body = request_articles(&query_info).expect("Error parsing JSON response");
    let soup = Soup::new(&body.content.html);
    let articles = extract_articles(&soup);

    Ok(articles)
}

fn extract_articles(soup: &Soup) -> Vec<Article> {
    let mut articles: Vec<Article> = vec![];

    for article in soup.tag("p").class("item").find_all() {
        let mut parents = article.parents();
        let mut link: String = String::from(BASE_URL);

        let a_tag = parents.find(|tag| tag.name().to_string() == "a");
        let href = a_tag.unwrap().get("href").unwrap();

        link.push_str(&href);

        let raw_title = String::from(&article.text());
        let title = raw_title.trim().replace('\u{a0}', " ");

        articles.push(Article { title, link });
    }

    articles
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    fn response_from_mock(mock_name: &str) -> BoletinResponse {
        let mut file_name = String::from("src/boletin/mocks/");
        file_name.push_str(mock_name);
        let raw = fs::read_to_string(file_name).expect("Unable to read file");
        serde_json::from_str(&raw).expect("Unable to parse json file")
    }

    fn soup_from_response(boletin_response: BoletinResponse) -> Soup {
        let content = &boletin_response.content.html;
        Soup::new(content)
    }

    #[test]
    fn extact_single_article() {
        let title = String::from("Testing Title");
        let link = String::from("https://www.boletinoficial.gob.ar/testing_title_link");
        let article = Article { title, link };
        let response = response_from_mock("single-result-response.json");
        let soup = soup_from_response(response);
        assert_eq!(extract_articles(&soup), vec![article]);
    }

    #[test]
    fn extract_empty_string_on_empty_response() {
        let response = response_from_mock("empty-response.json");
        let soup = soup_from_response(response);
        assert_eq!(extract_articles(&soup), vec![]);
    }

    #[test]
    fn extract_multiple_articles_single_page() {
        let response = response_from_mock("single-page-multi-result-response.json");
        let soup = soup_from_response(response);

        let title = String::from("PRESUPUESTO");
        let link = String::from(
            "https://www.boletinoficial.gob.ar/detalleAviso/primera/247706/20210805?busqueda=1",
        );
        let article_1 = Article { title, link };

        let title = String::from("POLICÍA DE SEGURIDAD AEROPORTUARIA");
        let link = String::from(
            "https://www.boletinoficial.gob.ar/detalleAviso/primera/247743/20210805?busqueda=1",
        );
        let article_2 = Article { title, link };

        let title = String::from("SUBSECRETARÍA DE INVESTIGACIÓN CRIMINAL Y COOPERACIÓN JUDICIAL");
        let link = String::from(
            "https://www.boletinoficial.gob.ar/detalleAviso/primera/247653/20210803?busqueda=1",
        );
        let article_3 = Article { title, link };

        let articles = vec![article_1, article_2, article_3];
        assert_eq!(extract_articles(&soup), articles);
    }

    #[test]
    fn extract_first_page_multi_page() {
        let response = response_from_mock("multi-page-multi-result-response.json");
        let soup = soup_from_response(response);

        let extracted = extract_articles(&soup);

        assert_eq!(extracted.len(), 100);
    }
}
