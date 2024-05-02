pub fn cycle_sort<T: Ord + Copy>(array: &mut [T]) {
    let array_length = array.len();
    for start_index in 0..array_length {
        let mut item_to_sort = array[start_index];
        let mut item_position = start_index;
        for index in start_index + 1..array_length {
            if array[index] < item_to_sort {
                item_position += 1;
            }
        }

        // Skip if item is already in the correct position
        if item_position == start_index {
            continue;
        }

        // Move item to its correct position
        while item_to_sort == array[item_position] {
            item_position += 1;
        }
        array.swap(item_position, start_index);
        item_to_sort = array[item_position]; // Update the item after swap

        // Rotate remaining cycle
        while item_position != start_index {
            item_position = start_index;
            for index in start_index + 1..array_length {
                if array[index] < item_to_sort {
                    item_position += 1;
                }
            }

            while item_position < array_length && item_to_sort == array[item_position] {
                item_position += 1;
            }

            if item_position != start_index {
                array.swap(item_position, start_index);
                item_to_sort = array[item_position]; // Update the item after swap
            }
        }
    }
}
