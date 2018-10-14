extern crate clap;

use clap::{App, Arg, SubCommand};

fn main() {
    let matches = App::new("Calculator")
        .subcommand(SubCommand::with_name("add") 
            .about("Sum some numbers")
            .version("0.1") 
            .author("Jean-Marcel Belmont")
            .arg(Arg::with_name("numbers")
                .multiple(true)
                .help("the numbers to add")
                .required(true)))
        .subcommand(SubCommand::with_name("subtract") 
            .about("Subtract some numbers")
            .version("0.1") 
            .author("Jean-Marcel Belmont")
            .arg(Arg::with_name("numbers")
                .multiple(true)
                .help("the numbers to subtract")
                .index(1)
                .required(true)))
        .subcommand(SubCommand::with_name("multiply") 
            .about("Multiply some numbers")
            .version("0.1") 
            .author("Jean-Marcel Belmont")
            .arg(Arg::with_name("numbers")
                .multiple(true)
                .help("the numbers to multiply")
                .index(1)
                .required(true)))
        .subcommand(SubCommand::with_name("divide") 
            .about("Divide some numbers")
            .version("0.1") 
            .author("Jean-Marcel Belmont")
            .arg(Arg::with_name("numbers")
                .multiple(true)
                .help("the numbers to divide")
                .index(1)
                .required(true)))
        .get_matches();

    // You can check if a subcommand was used like normal
    if matches.is_present("add") {
        println!("'calculator add' was run.");
    } else if matches.is_present("subtract") {
        println!("'calculator subtract' was run.");
    } else if matches.is_present("multiply") {
        println!("'calculator multiply' was run.");
    } else if matches.is_present("divide") {
        println!("'calculator divide' was run.");
    } else {
        println!("no subcommand was used");
    }

    // You can also match on a subcommand's name
    match matches.subcommand() {
        ("add", Some(add_matches)) =>{
            // Now we have a reference to add's matches
            let numbers = add_matches.values_of("numbers").unwrap().collect::<Vec<_>>().join(", ");
            println!("numbers is {}", numbers);
        },
        ("subtract", Some(subtract_matches)) =>{
            // Now we have a reference to add's matches
            let numbers = subtract_matches.values_of("numbers").unwrap().collect::<Vec<_>>().join(", ");
            println!("numbers is {}", numbers);
        },
        ("multiply", Some(multiply_matches)) =>{
            // Now we have a reference to add's matches
            let numbers = multiply_matches.values_of("numbers").unwrap().collect::<Vec<_>>().join(", ");
            println!("numbers is {}", numbers);
        },
        ("divide", Some(divide_matches)) =>{
            // Now we have a reference to add's matches
            let numbers = divide_matches.values_of("numbers").unwrap().collect::<Vec<_>>().join(", ");
            println!("numbers is {}", numbers);
        }
        ("", None)   => println!("No subcommand was used"),
        _            => unreachable!(),
    }
}