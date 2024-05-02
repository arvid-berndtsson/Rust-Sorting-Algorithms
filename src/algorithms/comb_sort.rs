pub fn comb_sort<T: Ord>(array: &mut [T]) {
    let mut gap_size = array.len();
    let shrink_factor = 1.3;
    let mut is_sorted = false;

    while !is_sorted {
        // Update the gap size for the next comb
        gap_size = (gap_size as f64 / shrink_factor).floor() as usize;

        if gap_size <= 1 {
            gap_size = 1;
            is_sorted = true;
        }

        let mut i = 0;
        while i + gap_size < array.len() {
            if array[i] > array[i + gap_size] {
                array.swap(i, i + gap_size);
                is_sorted = false;
            }
            i += 1;
        }
    }
}
