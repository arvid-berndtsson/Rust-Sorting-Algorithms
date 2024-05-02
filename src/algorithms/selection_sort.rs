pub fn selection_sort(numbers: &mut [i32]) {
    for current_index in 0..numbers.len() {
        let mut smallest_number_index = current_index;
        for next_index in (current_index + 1)..numbers.len() {
            if numbers[next_index] < numbers[smallest_number_index] {
                smallest_number_index = next_index;
            }
        }
        numbers.swap(current_index, smallest_number_index);
    }
}
