extern crate average;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let numbers = &args[1..];

    let numbers: Vec<f32> = numbers
        .iter()
        .map(|num| num.parse().unwrap())
        .collect();

    println!("{:?}", average::average(numbers));
}
