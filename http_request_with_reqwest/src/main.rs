extern crate reqwest;

use std::error::Error;
use std::io::Read;
use std::process;
use std::env;


// `Result` is a type that represents either success ([`Ok`]) or failure ([`Err`])
// `Box` A pointer type for heap allocation.
// `Error` is a trait representing the basic expectations for error values
fn run() -> Result<(), Box<dyn Error>> {
    let travis_token = env::var("TRAVIS_PERSONAL_TOKEN").unwrap().to_string();
    let client = reqwest::Client::builder()
        .build()?;
    let mut res = client
        .get("https://api.travis-ci.org/repos")
        .header("Travis-API-Version", "3")
        .header("Authorization", "token ".to_string() + &travis_token)
        .send()?;
    let mut body = String::new();
    res.read_to_string(&mut body)?;

    println!("Status: {}", res.status());
    println!("Headers:\n{:#?}", res.headers());
    println!("Body:\n{}", body);

    Ok(())
}

fn main() {
    if let Err(e) = run() {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}
