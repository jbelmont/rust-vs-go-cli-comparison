use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let numbers = &args[1];

    println!("{}", numbers);
}