#[macro_use] extern crate prettytable;

extern crate reqwest;
extern crate select;

use std::error::Error;
use std::process;

use select::document::Document;
use select::predicate::Attr;
use prettytable::Table;

fn run() -> Result<(), Box<dyn Error>> {
    // Web scraping my own blog post :)
    let res = reqwest::get("https://www.marcelbelmont.com/post/ci-cd-book/").unwrap();
    assert!(res.status().is_success());

    let document = Document::from_read(res).unwrap();

    let mut table = Table::new();

    let id_selector = "hands-on-continuous-integration-and-delivery";
    
    for node in document.find(Attr("id", id_selector)) {
        let title = node.text();
        // Fy-> is formatting syntax specific to prettytable
        table.add_row(row![Fy->title]);
    }
    table.printstd();

    Ok(())
}

fn main() {
    if let Err(e) = run() {
        eprintln!("Error trying to scrape my blog: {}", e);
        process::exit(1);
    }
}
