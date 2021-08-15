// Send Request
// Parse response
// Find articles in soup
    // TODO: use soup crate https://docs.rs/soup/0.5.1/soup/ https://crates.io/crates/soup
    // for parsing


// optionally print content
// send email with content

use soup::prelude::*;
use reqwest::header::{HeaderMap, HeaderValue, USER_AGENT, CONTENT_TYPE, ACCEPT_ENCODING, ACCEPT, CONNECTION};
use serde::Deserialize;
use std::collections::HashMap;

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

#[derive(Debug)]
struct Article {
    title: String,
    link: String,
}

pub fn post_query() -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::blocking::Client::new();

    let data = r#"
        {
            "busquedaRubro": false,
            "hayMasResultadosBusqueda": true,
            "ejecutandoLlamadaAsincronicaBusqueda": false,
            "ultimaSeccion": "",
            "filtroPorRubrosSeccion": false,
            "filtroPorRubroBusqueda": false,
            "filtroPorSeccionBusqueda": false,
            "busquedaOriginal": true,
            "ordenamientoSegunda": false,
            "seccionesOriginales": [1, 2, 3],
            "ultimoItemExterno": null,
            "ultimoItemInterno": null,
            "texto": "ANAC",
            "rubros": [],
            "nroNorma": "",
            "anioNorma": "",
            "denominacion": "",
            "tipoContratacion": "",
            "anioContratacion": "",
            "nroContratacion": "",
            "fechaDesde": "21/07/2021",
            "fechaHasta": "21/07/2021",
            "todasLasPalabras": true,
            "comienzaDenominacion": true,
            "seccion": [1, 2, 3],
            "tipoBusqueda": "Avanzada",
            "numeroPagina": 1,
            "ultimoRubro": ""
        }"#;

    let mut query = String::from("params=");
    query.push_str(data);
    query.push_str("&array_volver=[]");
    query.retain(|c| !c.is_whitespace());


    fn construct_headers() -> HeaderMap {
        let mut headers = HeaderMap::new();
        headers.insert(USER_AGENT, HeaderValue::from_static("Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_5) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/84.0.4147.89 Safari/537.36"));
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/x-www-form-urlencoded; charset=UTF-8"));
        headers.insert(ACCEPT_ENCODING, HeaderValue::from_static("gzip, deflate"));
        headers.insert(ACCEPT, HeaderValue::from_static("*/*"));
        headers.insert(CONNECTION, HeaderValue::from_static("keep-alive"));

        headers
    }

    let res = client.post(POST_URL)
        .headers(construct_headers())
        .body(query)
        .send()?;

    // For debugging
    // let soup = Soup::from_reader(res)?.text();
    // println!("The response is: {}", soup);
    // println!("The query is: {}", query);
    // println!("The query is: {:?}", data);
    // println!("The request is: {:?}", res);
    // println!("{}", query);

    let body = res.json::<BoletinResponse>()?;
    let content = &body.content;

    let soup = Soup::new(&content.html);
    let articles = soup.tag("p").class("item").find_all();

    for (i, article) in articles.enumerate() {

        let mut parents = article.parents();
        let anchor = parents.find(|tag| is_link(&tag.name().to_string()));
        let mut link: String = String::from(BASE_URL);

        if let Some(a) = anchor {
            let l = a.get("href");
            if let Some(href) = l {
                link.push_str(&href);
            }
        }

        let raw_title = String::from(&article.text());

        // title.remove_matches('\u{a0}');
        let title = raw_title.trim().replace('\u{a0}', " ");

        let a = Article { title, link};
        println!("New article #{}: {:?}", i, a);
    }

    // println!("The html content is: {:?}", &content.html);

    // println!("The first p is: {:?}", p.get("id"));



    // from python:  # soup.findAll('p', {'class': 'item'})
    // for ps in soup.tag("p").find_all() {
    //     println!("the ps are: {}", ps.display())
    // }


    Ok(())
}

fn is_link(tag: &str) -> bool {
            tag == "a"
        }
