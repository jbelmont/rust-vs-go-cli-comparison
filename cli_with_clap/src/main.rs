extern crate clap;

use clap::{App, Arg, SubCommand};

fn main() {
    let matches = App::new("Calculator")
        .subcommand(SubCommand::with_name("add") 
            .about("Sum 2 numbers")
            .version("0.1") 
            .author("Jean-Marcel Belmont")
            .arg(Arg::with_name("input")
                .help("the file to add")
                .index(1)
                .required(true)))
        .get_matches();

    // You can check if a subcommand was used like normal
    if matches.is_present("add") {
        println!("'myapp add' was run.");
    }

    // You can get the independent subcommand matches (which function exactly like App matches)
    if let Some(matches) = matches.subcommand_matches("add") {
        // Safe to use unwrap() because of the required() option
        println!("Adding file: {}", matches.value_of("input").unwrap());
    }

    // You can also match on a subcommand's name
    match matches.subcommand_name() {
        Some("add") => println!("'myapp add' was used"),
        None        => println!("No subcommand was used"),
        _           => println!("Some other subcommand was used"),
    }

    // Continued program logic goes here...
}