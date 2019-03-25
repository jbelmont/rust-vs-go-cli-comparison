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
        .subcommand(SubCommand::with_name("remainder")
            .about("Remainder between 2 numbers")
            .version("0.2")
            .author("Jean-Marcel Belmont")
            .arg(Arg::with_name("numbers")
                .multiple(true)
                .help("the numbers to find remainder of")
                .index(1)
                .required(true)))
        .get_matches();

    // You can also match on a subcommand's name
    let mut value = 0.0;
    match matches.subcommand() {
        ("add", Some(add_matches)) => {
            // Now we have a reference to add's matches
            let mut sum = 0.0;
            let numbers = add_matches.values_of("numbers").unwrap().collect::<Vec<_>>();
            for num in numbers.iter() {
                if *num == String::from("number") {
                    continue
                } else {
                    sum += num.parse::<f32>().unwrap();
                }
            }
            value = sum;
        },
        ("subtract", Some(subtract_matches)) => {
            // Now we have a reference to add's matches
            let numbers = subtract_matches.values_of("numbers").unwrap().collect::<Vec<_>>();
            let mut subtrahend = 0.0;
            for num in numbers.iter() {
                if *num == String::from("number") {
                    continue
                } else {
                    subtrahend -= num.parse::<f32>().unwrap();
                }
            }
            value = subtrahend;
        },
        ("multiply", Some(multiply_matches)) => {
            // Now we have a reference to add's matches
            let numbers = multiply_matches.values_of("numbers").unwrap().collect::<Vec<_>>();
            let mut product = 1.0;
            for num in numbers.iter() {
                if *num == String::from("number") {
                    continue
                } else {
                    product *= num.parse::<f32>().unwrap();
                }
            }
            value = product;
        },
        ("divide", Some(divide_matches)) => {
            // Now we have a reference to add's matches
            let numbers = divide_matches.values_of("numbers").unwrap().collect::<Vec<_>>();
            let mut dividend = 1.0;
            for num in numbers.iter() {
                if *num == String::from("number") {
                    continue
                } else {
                    if num.parse::<i32>().unwrap() == 0 {
                        continue
                    }
                    dividend /= num.parse::<f32>().unwrap();
                }
            }
            value = dividend;
        },
        ("remainder", Some(remainder_matches)) => {
            // Now we have a reference to add's matches
            let numbers = remainder_matches.values_of("numbers").unwrap().collect::<Vec<_>>();
            let quotient = numbers[0].parse::<i32>().unwrap();
            let divisor = numbers[1].parse::<i32>().unwrap();
            let remainder = quotient % divisor;
            value = remainder as f32;
        },
        ("", None)   => println!("No subcommand was used"),
        _            => unreachable!(),
    }
    println!("The calculation is {}", value);
}
