pub fn library_sort<T: Ord + Copy>(input_array: &mut [T]) {
    let mut expanded_array: Vec<Option<T>> = vec![None; input_array.len() * 2];
    for &current_item in input_array.iter() {
        let mut insertion_index = match expanded_array.iter().position(|&existing_item| {
            existing_item == None || existing_item.unwrap() > current_item
        }) {
            Some(x) => x,
            None => expanded_array.len() - 1,
        };
        while expanded_array[insertion_index].is_some() {
            insertion_index += 1;
        }
        expanded_array[insertion_index] = Some(current_item);
    }
    let sorted_array: Vec<T> = expanded_array.into_iter().filter_map(|x| x).collect();
    input_array.copy_from_slice(&sorted_array);
}
