extern crate hyper;

use std::env;

use hyper::rt::{Future, Stream};
use hyper::{Body, Client, Chunk, Method, Request, Response};


static LOWERCASE: &[u8] = b"lowercase string";

fn main() {

    let travis_token = env::var("TRAVIS_PERSONAL_TOKEN").unwrap().to_string();

    let req = Request::builder()
        .uri("https://api.travis-ci.org/repos")
        .method(Method::GET)
        .header("Travis-API-Version", "3")
        .header("Authorization", "token ".to_string() + &travis_token)
        .body(LOWERCASE.into())
        .unwrap();

    // use the request with client
    let client = Client::new();

    client.request(req)
        .map(|web_res| {
            // return the response that came from the web api and the original text together
            // to show the difference
        let body = Body::wrap_stream(web_res.into_body().map(|b| {
            Chunk::from(format!("<b>before</b>: {}<br><b>after</b>: {}",
                                std::str::from_utf8(LOWERCASE).unwrap(),
                                std::str::from_utf8(&b).unwrap()))
        }));

        Response::new(body)
    });
}