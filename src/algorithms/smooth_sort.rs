pub fn smooth_sort<T: Ord + Copy>(input_array: &mut [T]) {
    let array_length = input_array.len();
    let mut heap_size = 0;
    let mut index = 0;

    while index < array_length {
        if (heap_size & 3) == 3 {
            sift(input_array, index - 1, heap_size >> 2);
            heap_size = (heap_size >> 2) + 1;
        } else if index + 1 < array_length {
            sift(input_array, index, heap_size);
            heap_size = (heap_size << 2) + 3;
            index += 1;
        } else {
            trinkle(input_array, index, heap_size, false);
            break;
        }
        index += 1;
    }

    while heap_size > 1 {
        heap_size -= 1;
        if index == 0 {
            break;
        }
        trinkle(input_array, index - 1, heap_size, false);
        index -= 1;
    }
}

fn sift<T: Ord + Copy>(input_array: &mut [T], mut sift_index: usize, mut sift_heap: usize) {
    let sift_value = input_array[sift_index];
    while sift_heap > 1 {
        let sift_heap_parent = sift_heap >> 1;
        if sift_heap_parent > sift_index {
            break;
        }
        let sift_heap_child = sift_index - sift_heap_parent;
        if sift_value >= input_array[sift_heap_child] {
            break;
        }
        input_array[sift_index] = input_array[sift_heap_child];
        sift_index = sift_heap_child;
        sift_heap = sift_heap_parent;
    }
    input_array[sift_index] = sift_value;
}

fn trinkle<T: Ord + Copy>(
    input_array: &mut [T],
    mut trinkle_index: usize,
    mut trinkle_heap: usize,
    mut is_trust: bool,
) {
    let trinkle_value = input_array[trinkle_index];
    while trinkle_heap > 1 {
        let trinkle_heap_parent = trinkle_heap >> 1;
        if trinkle_heap_parent > trinkle_index {
            break;
        }
        let trinkle_heap_child = trinkle_index - trinkle_heap_parent;
        if trinkle_value < input_array[trinkle_heap_child]
            || (is_trust
                && trinkle_heap_child == trinkle_index - 1
                && input_array[trinkle_index - 1] <= input_array[trinkle_index])
        {
            input_array[trinkle_index] = input_array[trinkle_heap_child];
            trinkle_index = trinkle_heap_child;
            trinkle_heap = trinkle_heap_parent;
            is_trust = false;
        } else {
            break;
        }
    }
    input_array[trinkle_index] = trinkle_value;
}
