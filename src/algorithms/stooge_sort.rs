pub fn stooge_sort<T: Ord>(unsorted_array: &mut [T]) {
    let array_length = unsorted_array.len();
    stooge(unsorted_array, 0, array_length);
}

fn stooge<T: Ord>(unsorted_array: &mut [T], start_index: usize, end_index: usize) {
    if end_index - start_index > 1 {
        if unsorted_array[start_index] > unsorted_array[end_index - 1] {
            unsorted_array.swap(start_index, end_index - 1);
        }
        if end_index - start_index > 2 {
            let one_third_length = (end_index - start_index) / 3;
            stooge(unsorted_array, start_index, end_index - one_third_length);
            stooge(unsorted_array, start_index + one_third_length, end_index);
            stooge(unsorted_array, start_index, end_index - one_third_length);
        }
    }
}
