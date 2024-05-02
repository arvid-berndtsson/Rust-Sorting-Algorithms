pub fn merge_sort(numbers: &mut [i32]) {
    let middle_index = numbers.len() / 2;
    if middle_index == 0 {
        return;
    }

    merge_sort(&mut numbers[..middle_index]);
    merge_sort(&mut numbers[middle_index..]);

    let mut sorted_numbers: Vec<i32> = numbers.to_vec();

    merge(
        &numbers[..middle_index],
        &numbers[middle_index..],
        &mut sorted_numbers[..],
    );

    numbers.copy_from_slice(&sorted_numbers);
}

fn merge(left_half: &[i32], right_half: &[i32], sorted_numbers: &mut [i32]) {
    let (mut left_index, mut right_index, mut sorted_index) = (0, 0, 0);

    while left_index < left_half.len() && right_index < right_half.len() {
        if left_half[left_index] <= right_half[right_index] {
            sorted_numbers[sorted_index] = left_half[left_index];
            left_index += 1;
        } else {
            sorted_numbers[sorted_index] = right_half[right_index];
            right_index += 1;
        }
        sorted_index += 1;
    }

    if left_index < left_half.len() {
        sorted_numbers[sorted_index..].copy_from_slice(&left_half[left_index..]);
    }
    if right_index < right_half.len() {
        sorted_numbers[sorted_index..].copy_from_slice(&right_half[right_index..]);
    }
}
