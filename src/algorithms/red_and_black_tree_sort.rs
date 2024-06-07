use std::collections::BTreeMap;

pub fn red_black_tree_sort(input_array: &mut [i32]) {
    let mut number_frequency_map = BTreeMap::new();
    for &current_number in input_array.iter() {
        *number_frequency_map.entry(current_number).or_insert(0) += 1;
    }
    let mut sorted_array_index = 0;
    for (&unique_number, &frequency) in number_frequency_map.iter() {
        for _ in 0..frequency {
            input_array[sorted_array_index] = unique_number;
            sorted_array_index += 1;
        }
    }
}