extern crate summation_with_tests;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let numbers = &args[1..];

    let numbers: Vec<f32> = numbers
        .iter()
        .map(|num| num.parse().unwrap())
        .collect();

    let summation = summation_with_tests::summation_with_fold(numbers.clone());

    println!("summation_with_fold is {}", summation);

    let summation = summation_with_tests::summation_with_loop(numbers.clone());

    println!("summation_with_loop is {}", summation);
}