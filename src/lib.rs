use std::collections::HashMap;

pub fn list_median(numbers: &Vec<i32>) -> i32 {
    let size = numbers.len();

    if size == 0 {
        return 0
    }

    let index = (size / 2) - 1;
    let mut sorted_numbers = numbers.to_owned();
    sorted_numbers.sort();

    if size % 2 != 0 {
        return sorted_numbers[index + 1]
    }

    (sorted_numbers[index] + sorted_numbers[index + 1]) / 2
}

pub fn list_mode(numbers: &Vec<i32>) -> HashMap<&i32, i32> {
    let mut numbers_count = HashMap::new();

    for number in numbers {
        let count = numbers_count.entry(number).or_insert(0);

        *count += 1;
    }

    numbers_count
}

#[cfg(test)]
mod tests {
    use super::*;

    // Median
    #[test]
    fn test_median_empty() {
        assert_eq!(list_median(&Vec::new()), 0);
    }

    #[test]
    fn test_median_odd() {
        assert_eq!(list_median(&vec![43, 12, 25, 4, 11]), 12);
    }

    #[test]
    fn test_median_even() {
        assert_eq!(list_median(&vec![100, 90, 20, 130]), 95);
    }

    // Mode
    #[test]
    fn test_mode_empty() {
        assert_eq!(list_mode(&Vec::new()), HashMap::new());
    }

    #[test]
    fn test_mode_list() {
        let numbers = vec![4, 4, 1, 4, 5];
        let mut right = HashMap::new();

        right.insert(&numbers[0], 3);
        right.insert(&numbers[2], 1);
        right.insert(&numbers[4], 1);

        assert_eq!(list_mode(&numbers), right);
    }
}



