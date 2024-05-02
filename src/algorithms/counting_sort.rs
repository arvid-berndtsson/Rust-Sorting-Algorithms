pub fn counting_sort(input_array: &mut [i32]) {
    if input_array.is_empty() {
        return;
    }
    let min_value = *input_array.iter().min().unwrap() as usize;
    let max_value = *input_array.iter().max().unwrap() as usize;
    let range = max_value - min_value + 1;

    let mut count_array = vec![0; range];
    let mut sorted_array = vec![0; input_array.len()];

    for &number in input_array.iter() {
        count_array[(number as usize) - min_value] += 1;
    }

    for i in 1..range {
        count_array[i] += count_array[i - 1];
    }

    for &number in input_array.iter().rev() {
        sorted_array[count_array[(number as usize) - min_value] - 1] = number;
        count_array[(number as usize) - min_value] -= 1;
    }

    input_array.copy_from_slice(&sorted_array);
}
