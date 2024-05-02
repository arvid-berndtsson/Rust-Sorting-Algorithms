pub fn shell_sort<T: Ord + Copy>(input_array: &mut [T]) {
    let array_length = input_array.len();
    let mut gap_size = array_length / 2;
    while gap_size > 0 {
        for current_index in gap_size..array_length {
            let temp_value = input_array[current_index];
            let mut previous_index = current_index;
            while previous_index >= gap_size && input_array[previous_index - gap_size] > temp_value
            {
                input_array[previous_index] = input_array[previous_index - gap_size];
                previous_index -= gap_size;
            }
            input_array[previous_index] = temp_value;
        }
        gap_size /= 2;
    }
}
