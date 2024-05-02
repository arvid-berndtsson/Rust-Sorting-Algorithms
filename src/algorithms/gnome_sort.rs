pub fn gnome_sort<T: Ord>(array: &mut [T]) {
    let mut current_index = 0;
    while current_index < array.len() {
        if current_index == 0 || array[current_index] >= array[current_index - 1] {
            current_index += 1;
        } else {
            array.swap(current_index, current_index - 1);
            current_index -= 1;
        }
    }
}
