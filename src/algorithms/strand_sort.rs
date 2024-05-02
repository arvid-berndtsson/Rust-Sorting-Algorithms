pub fn strand_sort<T: Ord + Clone>(unsorted_list: &mut [T]) {
    let mut sorted_list: Vec<T> = Vec::new();
    let mut unsorted_list = unsorted_list.to_vec();

    while !unsorted_list.is_empty() {
        let mut extracted_sorted_sublist: Vec<T> = Vec::new();
        let mut current_index = 0;

        while current_index < unsorted_list.len() {
            if extracted_sorted_sublist.is_empty()
                || &unsorted_list[current_index] >= extracted_sorted_sublist.last().unwrap()
            {
                extracted_sorted_sublist.push(unsorted_list.remove(current_index));
            } else {
                current_index += 1;
            }
        }

        merge_sorted_sublist_into_sorted_list(&mut sorted_list, extracted_sorted_sublist);
    }

    unsorted_list.clear();
    unsorted_list.extend(sorted_list);
}

fn merge_sorted_sublist_into_sorted_list<T: Ord + Clone>(
    sorted_list: &mut Vec<T>,
    mut sorted_sublist: Vec<T>,
) {
    let mut sorted_list_index = 0;
    let sorted_sublist_index = 0;

    while sorted_list_index < sorted_list.len() && sorted_sublist_index < sorted_sublist.len() {
        if &sorted_sublist[sorted_sublist_index] < &sorted_list[sorted_list_index] {
            sorted_list.insert(
                sorted_list_index,
                sorted_sublist.remove(sorted_sublist_index),
            );
        } else {
            sorted_list_index += 1;
        }
    }

    if sorted_sublist_index < sorted_sublist.len() {
        sorted_list.append(&mut sorted_sublist);
    }
}
