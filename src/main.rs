use reqwest::blocking;

fn main() {
    // let _args: Vec<String> = env::args().collect();

    let client = blocking::Client::new();
    let res = client.post("http://httpbin.org/post")
    .body("the exact body that is sent")
    .send();

    println!("{:?}", res);
}

// TODO: use soup crate https://docs.rs/soup/0.5.1/soup/ https://crates.io/crates/soup
// for parsing
