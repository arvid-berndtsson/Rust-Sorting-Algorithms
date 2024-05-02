pub fn bitonic_sort<T: Ord>(array: &mut [T], ascending: bool) {
    let array_size = array.len();
    if array_size <= 1 {
        return;
    }

    let middle_index = array_size / 2;
    bitonic_sort(&mut array[..middle_index], true);
    bitonic_sort(&mut array[middle_index..], false);
    bitonic_merge(array, ascending);
}

pub fn bitonic_merge<T: Ord>(array: &mut [T], ascending: bool) {
    let array_size = array.len();
    if array_size <= 1 {
        return;
    }

    let middle_index = array_size / 2;
    for index in 0..middle_index {
        if (array[index] > array[index + middle_index]) == ascending {
            array.swap(index, index + middle_index);
        }
    }

    bitonic_merge(&mut array[..middle_index], ascending);
    bitonic_merge(&mut array[middle_index..], ascending);
}
