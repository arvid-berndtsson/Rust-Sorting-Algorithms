pub fn strand_sort<T: Ord + Clone>(list: &mut [T]) {
    let mut input = list.to_vec();
    let mut sorted_list = Vec::new();

    while !input.is_empty() {
        let mut sorted_sublist = vec![input.remove(0)];

        input.retain(|item| {
            if item >= sorted_sublist.last().unwrap() {
                sorted_sublist.push(item.clone());
                false
            } else {
                true
            }
        });

        merge_sorted_sublist_into_sorted_list(&mut sorted_list, sorted_sublist);
    }

    list.clone_from_slice(&sorted_list[..]);
}

fn merge_sorted_sublist_into_sorted_list<T: Ord + Clone>(
    sorted_list: &mut Vec<T>,
    mut sorted_sublist: Vec<T>,
) {
    let mut sorted_list_index = 0;

    while sorted_list_index < sorted_list.len() && !sorted_sublist.is_empty() {
        if &sorted_sublist[0] < &sorted_list[sorted_list_index] {
            sorted_list.insert(sorted_list_index, sorted_sublist.remove(0));
        } else {
            sorted_list_index += 1;
        }
    }

    sorted_list.append(&mut sorted_sublist);
}
