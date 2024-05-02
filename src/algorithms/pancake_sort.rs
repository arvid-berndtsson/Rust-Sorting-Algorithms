pub fn pancake_sort<T: Ord + Copy>(array: &mut [T]) {
    for unsorted_length in (1..=array.len()).rev() {
        // Find index of the maximum element in array[0..unsorted_length]
        let mut max_element_index = 0;
        let mut max_element_value = array[0];
        for index in 1..unsorted_length {
            if array[index] > max_element_value {
                max_element_value = array[index];
                max_element_index = index;
            }
        }

        // Swap the maximum element with the last element using pancake flip
        if max_element_index != unsorted_length - 1 {
            // Flip array[0..max_element_index+1]
            array[0..=max_element_index].reverse();
            // Flip array[0..unsorted_length]
            array[0..unsorted_length].reverse();
        }
    }
}
