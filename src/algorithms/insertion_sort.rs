pub fn insertion_sort(numbers: &mut [i32]) {
    for index in 1..numbers.len() {
        let mut current_index = index;
        while current_index > 0 && numbers[current_index - 1] > numbers[current_index] {
            numbers.swap(current_index, current_index - 1);
            current_index -= 1;
        }
    }
}
