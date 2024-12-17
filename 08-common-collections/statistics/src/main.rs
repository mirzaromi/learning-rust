use std::collections::HashMap;

fn main() {
    let mut values = vec![1 ,5 ,2, 5, 7, 4, 8, 3, 5, 2, 6, 7, 4, 5, 9];

    values.sort();

    let total_data = values.len();

    let median;
    if total_data % 2 == 0 {
        median = (values[total_data/2] + values[(total_data/2)+1])/2;
    } else {
        median = values[(total_data+1)/2];
    }

    let mut hash_map = HashMap::new();

    let mut mode = 0;
    let mut mode_count = 0;

    for value in &values {
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

    println!("values : {:?}", values);
    println!("len : {}", values.len());
    println!("median : {}", median);
    println!("{:?}", hash_map);
    println!("mode_count: {}", mode_count);
    println!("mode: {}", mode);
    println!("median: {}", median);
}

