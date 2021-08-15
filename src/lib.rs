// Send Request
// Parse response
// Find articles in soup
    // TODO: use soup crate https://docs.rs/soup/0.5.1/soup/ https://crates.io/crates/soup
    // for parsing


// optionally print content
// send email with content

use soup::prelude::*;

pub fn post_query() -> Result<String, Box<dyn std::error::Error>> {
    let client = reqwest::blocking::Client::new();

    // let res = client.post("http://localhost:3000")
    let res = client.post("http://httpbin.org/post")
    .body("the exact body that is sent")
    .send()?;

    // For debugging
    // let soup = Soup::from_reader(res)?.text();
    // println!("The response is: {}", soup);

    let soup = Soup::from_reader(res)?;

    // from python:  # soup.findAll('p', {'class': 'item'})
    for ps in soup.tag("p").find_all() {
        println!("the ps are: {}", ps.display())
    }


    Ok(String::from("this worked"))
}
