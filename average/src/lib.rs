pub fn average(numbers: Vec<f32>) -> f32 {
    numbers
        .iter()
        .fold(0.0, |sum, val| sum + val) / numbers.len() as f32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn average_should_compute_a_value() {
        let numbers = vec![
            1.0, 2.0, 3.0,
        ];
        assert_eq!(2.0, average(numbers));
    }
}
