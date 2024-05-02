pub fn quick_sort(numbers: &mut [i32], start_index: usize, end_index: usize) {
    if start_index < end_index {
        let pivot_index = partition(numbers, start_index, end_index);
        if pivot_index > 0 {
            quick_sort(numbers, start_index, pivot_index - 1);
        }
        quick_sort(numbers, pivot_index + 1, end_index);
    }
}

fn partition(numbers: &mut [i32], start_index: usize, end_index: usize) -> usize {
    let pivot_value = numbers[end_index];
    let mut smaller_elements_index = start_index;
    for current_index in start_index..end_index {
        if numbers[current_index] < pivot_value {
            numbers.swap(smaller_elements_index, current_index);
            smaller_elements_index += 1;
        }
    }
    numbers.swap(smaller_elements_index, end_index);
    smaller_elements_index
}
