fn median(numbers: Vec<i32>) -> i32 {
    let mut cloned_numbers = numbers.clone();

    cloned_numbers.sort();

    let middle_position: f64 = (cloned_numbers.len() as f64) / 2.0;
    let middle_index: usize = middle_position.floor() as usize;

    let median = cloned_numbers
        .get(middle_index)
        .expect("Cannot access middle index of numbers!");

    return *median;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_median() {
        let numbers: Vec<i32> = vec![9, 10, 12, 13, 13, 13, 15, 15, 16, 16, 18, 22, 23, 24, 24];
        let expected_median = 15;

        let median = median(numbers);

        assert_eq!(median, expected_median)
    }
}
