extern crate reqwest;

use std::process;

fn main() {
    if let Err(e) = http_request_with_reqwest::run() {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}
