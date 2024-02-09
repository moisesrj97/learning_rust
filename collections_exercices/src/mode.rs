use std::collections::HashMap;

fn mode(numbers: Vec<i32>) -> Vec<i32> {
    let mut numbers_map: HashMap<&i32, i32> = HashMap::new();

    for number in numbers.iter() {
        let count = numbers_map.entry(number).or_insert(0);
        *count += 1;
    }

    let mut numbers_with_most_repetitions: Vec<i32> = Vec::new();

    for pair in numbers_map.iter() {
        let existing_max = numbers_with_most_repetitions.get(0);

        let pair_key = **pair.0;
        let pair_value = pair.1;

        match existing_max {
            Some(existing) => {
                let value = numbers_map.get(existing).expect("Error");

                match pair_value.cmp(value) {
                    std::cmp::Ordering::Less => continue,
                    std::cmp::Ordering::Equal => (),
                    std::cmp::Ordering::Greater => numbers_with_most_repetitions = Vec::new(),
                }

                numbers_with_most_repetitions.push(pair_key);
            }
            None => numbers_with_most_repetitions.push(pair_key),
        }
    }

    numbers_with_most_repetitions.sort();

    return numbers_with_most_repetitions;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mode() {
        let numbers: Vec<i32> = vec![9, 10, 12, 13, 13, 13, 15, 15, 16, 16, 18, 22, 23, 24, 24];
        let expected_mode = vec![13];

        let mode = mode(numbers);

        assert_eq!(mode, expected_mode)
    }

    #[test]
    fn test_mode_multiples() {
        let numbers: Vec<i32> = vec![
            9, 10, 12, 13, 13, 13, 15, 15, 15, 16, 16, 18, 22, 23, 24, 24,
        ];
        let expected_mode = vec![13, 15];

        let mode = mode(numbers);

        assert_eq!(mode, expected_mode)
    }
}
