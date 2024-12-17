use std::collections::HashMap;

fn main() {
    // let mut values = vec![1 ,5 ,2, 5, 7, 4, 8, 3, 5, 2, 6, 7, 4, 5, 9];
    let values = vec![3, 3, 6, 7, 8, 8];

    let (median,mode) = find_median_mode(& values);

    println!("values : {:?}", values);
    println!("len : {}", values.len());
    println!("median : {}", median);
    println!("mode: {}", mode);
    println!("median: {}", median);
}

fn find_median_mode(values: & Vec<i32>) -> (f32, i32) {
    let mut sorted_values = values.clone();
    sorted_values.sort();

    let total_data = sorted_values.len();

    let median;
    if total_data % 2 == 0 {
        median = (sorted_values[total_data / 2 -1] + sorted_values[total_data / 2 ]) as f32 / 2.0;
    } else {
        median = sorted_values[(total_data) / 2] as f32;
    }

    let mut hash_map = HashMap::new();

    let mut mode = 0;
    let mut mode_count = 0;

    for value in &sorted_values {
        // fill the hash map
        let map_value = hash_map.entry(value)
            .and_modify(|val| *val += 1)
            .or_insert(1);

        // find mode
        if map_value > &mut mode_count {
            mode = *value;
            mode_count = *map_value;
        }
    }
    (median, mode)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_median_and_mode() {
        let numbers = vec![1, 2, 2, 3, 4];
        let (median, mode) = find_median_mode(&numbers);
        assert_eq!(median, 2.0); // Median of sorted [1, 2, 2, 3, 4]
        assert_eq!(mode, 2);    // Mode (most frequent) is 2

        let numbers2 = vec![3, 3, 6, 7, 8, 8];
        let (median2, mode2) = find_median_mode(&numbers2);
        assert_eq!(median2, 6.5); // Median of sorted [3, 3, 6, 7, 8, 8]
        assert_eq!(mode2, 3);     // Mode is 3 (appears first)

        let numbers3 = vec![10];
        let (median3, mode3) = find_median_mode(&numbers3);
        assert_eq!(median3, 10.0); // Single element median is itself
        assert_eq!(mode3, 10);     // Mode is itself
    }
}
