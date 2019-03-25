use std::process;

fn main() {
    if let Err(e) = web_scraping_with_rust::run() {
        eprintln!("Error trying to scrape my blog: {}", e);
        process::exit(1);
    }
}
