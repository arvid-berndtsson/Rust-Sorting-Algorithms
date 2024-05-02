pub fn patience_sort<T: Ord + Copy>(input_array: &mut [T]) {
    let mut pile_vector: Vec<Vec<T>> = Vec::new();
    for &current_number in input_array.iter() {
        let pile_index: usize =
            match pile_vector.binary_search_by(|pile: &Vec<T>| pile[0].cmp(&current_number)) {
                Ok(index) | Err(index) => index,
            };
        if pile_index == pile_vector.len() {
            pile_vector.push(vec![current_number]);
        } else {
            pile_vector[pile_index].push(current_number);
        }
    }
    for array_element in input_array.iter_mut() {
        let last_non_empty_pile: &mut Vec<T> = pile_vector
            .iter_mut()
            .rev()
            .find(|pile: &&mut Vec<T>| !pile.is_empty())
            .unwrap();
        *array_element = last_non_empty_pile.pop().unwrap();
        if last_non_empty_pile.is_empty() {
            pile_vector.pop();
        }
    }
}
