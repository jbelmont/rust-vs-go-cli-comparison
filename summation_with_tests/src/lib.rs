pub fn summation_with_loop(numbers: Vec<f32>) -> f32 {
    let mut sum = 0.0;
    for num in numbers.iter() {
        sum += num;
    }
    sum
}

pub fn summation_with_fold(numbers: Vec<f32>) -> f32 {
    let sum = numbers
        .iter()
        .fold(0.0, |sum, val| sum + val);
    return sum;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn summation_with_loop_should_compute_sum() {
        let numbers: Vec<f32> = vec![
            2.5,
            3.5,
            4.5,
        ];
        assert_eq!(
            summation_with_loop(numbers),
            10.5,
        );
    }

    #[test]
    fn summation_with_fold_should_compute_sum() {
        let numbers: Vec<f32> = vec![
            2.75,
            3.25,
            10.5
        ];
        assert_eq!(
            summation_with_fold(numbers),
            16.5
        );
    }
}