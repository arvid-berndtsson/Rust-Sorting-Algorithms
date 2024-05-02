pub fn radix_sort(input_array: &mut [i32]) {
    let max_number = match input_array.iter().map(|&x| x.abs() as u32).max() {
        Some(max_number) => max_number,
        None => return,
    };
    let mut digit_position = 1;
    while max_number / digit_position > 0 {
        counting_sort_by_digit(input_array, digit_position);
        digit_position *= 10;
    }
}

fn counting_sort_by_digit(input_array: &mut [i32], digit_position: u32) {
    let mut sorted_array: Vec<i32> = vec![0; input_array.len()];
    let mut digit_count: Vec<usize> = vec![0; 10];

    for &number in input_array.iter() {
        let digit_index = ((number.abs() as u32) / digit_position) % 10;
        digit_count[digit_index as usize] += 1;
    }

    for i in 1..10 {
        digit_count[i] += digit_count[i - 1];
    }

    for i in (0..input_array.len()).rev() {
        let number = input_array[i];
        let digit_index = ((number.abs() as u32) / digit_position) % 10;
        sorted_array[digit_count[digit_index as usize] - 1] = number;
        digit_count[digit_index as usize] -= 1;
    }

    for i in 0..input_array.len() {
        input_array[i] = sorted_array[i];
    }
}
