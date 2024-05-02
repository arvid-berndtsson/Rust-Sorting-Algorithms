pub fn tim_sort<T: Ord + Copy>(array_to_sort: &mut [T]) {
    let array_length = array_to_sort.len();
    let mut stack_of_runs: Vec<(usize, usize)> = Vec::new();

    let mut start_index = 0;
    while start_index < array_length {
        let mut end_index = start_index + 1;
        while end_index < array_length && array_to_sort[end_index - 1] <= array_to_sort[end_index] {
            end_index += 1;
        }

        let current_run = (start_index, end_index);
        while let Some(&(run_start, run_end)) = stack_of_runs.last() {
            if current_run.1 - current_run.0 > run_end - run_start {
                break;
            }

            stack_of_runs.pop();
            merge(array_to_sort, run_start, run_end, current_run.1);
        }

        stack_of_runs.push(current_run);
        start_index = end_index;
    }

    while let Some((run_start, run_end)) = stack_of_runs.pop() {
        merge(array_to_sort, run_start, run_end, array_length);
    }
}

fn merge<T: Ord + Copy>(array_to_sort: &mut [T], start: usize, mid: usize, end: usize) {
    let mut temporary_array = Vec::with_capacity(end - start);
    temporary_array.extend_from_slice(&array_to_sort[start..mid]);

    let (mut temp_index, mut right_index, mut sorted_index) = (0, mid, start);
    while temp_index < temporary_array.len() && right_index < end {
        if array_to_sort[right_index] < temporary_array[temp_index] {
            array_to_sort[sorted_index] = array_to_sort[right_index];
            right_index += 1;
        } else {
            array_to_sort[sorted_index] = temporary_array[temp_index];
            temp_index += 1;
        }
        sorted_index += 1;
    }

    while temp_index < temporary_array.len() {
        array_to_sort[sorted_index] = temporary_array[temp_index];
        temp_index += 1;
        sorted_index += 1;
    }
}
