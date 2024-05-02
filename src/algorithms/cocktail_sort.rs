pub fn cocktail_sort<T: Ord>(array: &mut [T]) {
    let mut start_index = 0;
    let mut end_index = array.len() - 1;

    while start_index < end_index {
        let mut new_end_index = start_index;
        let mut new_start_index = end_index;

        // Forward pass
        for i in start_index..end_index {
            if array[i] > array[i + 1] {
                array.swap(i, i + 1);
                new_end_index = i;
            }
        }

        end_index = new_end_index;

        // Backward pass
        for i in (start_index..end_index).rev() {
            if array[i] > array[i + 1] {
                array.swap(i, i + 1);
                new_start_index = i;
            }
        }

        start_index = new_start_index;
    }
}
