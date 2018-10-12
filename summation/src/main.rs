use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let numbers = &args[1..];

    let numbers: Vec<f32> = numbers
        .iter()
        .map(|num| num.parse().unwrap())
        .collect();

    let summation = summation_with_fold(numbers.clone());

    println!("summation_with_fold is {}", summation);

    let summation = summation_with_loop(numbers.clone());

    println!("summation_with_loop is {}", summation);
}

fn summation_with_loop(numbers: Vec<f32>) -> f32 {
    let mut sum = 0.0;
    for num in numbers.iter() {
        sum += num;
    }
    sum
}

fn summation_with_fold(numbers: Vec<f32>) -> f32 {
    let sum = numbers
        .iter()
        .fold(0.0, |sum, val| sum + val);
    return sum;
}