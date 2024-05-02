use std::time::Instant;

mod algorithms {
    pub mod bitonic_sort;
    pub mod bogo_sort;
    pub mod bubble_sort;
    pub mod cocktail_sort;
    pub mod comb_sort;
    pub mod counting_sort;
    pub mod cycle_sort;
    pub mod gnome_sort;
    pub mod heap_sort;
    pub mod insertion_sort;
    pub mod library_sort;
    pub mod merge_sort;
    pub mod pancake_sort;
    pub mod patience_sort;
    pub mod quick_sort;
    pub mod radix_sort;
    pub mod rb_tree_sort;
    pub mod selection_sort;
    pub mod shell_sort;
    pub mod sleep_sort;
    pub mod smooth_sort;
    pub mod stooge_sort;
    pub mod strand_sort;
    pub mod tim_sort;
    pub mod tree_sort;
}

fn main() {
    let unsorted_numbers = vec![34, 50, 23, 8, 23, 100, 4, 300554];

    let sorting_algorithms: Vec<(&str, Box<dyn Fn(&mut [i32])>)> = vec![
        (
            "Bitonic sort",
            Box::new(|numbers: &mut [i32]| algorithms::bitonic_sort::bitonic_sort(numbers, true)),
        ),
        (
            "Bogo sort",
            Box::new(|numbers: &mut [i32]| algorithms::bogo_sort::bogo_sort(numbers)),
        ),
        (
            "Bubble sort",
            Box::new(|numbers: &mut [i32]| algorithms::bubble_sort::bubble_sort(numbers)),
        ),
        (
            "Cocktail sort",
            Box::new(|numbers: &mut [i32]| algorithms::cocktail_sort::cocktail_sort(numbers)),
        ),
        (
            "Comb sort",
            Box::new(|numbers: &mut [i32]| algorithms::comb_sort::comb_sort(numbers)),
        ),
        (
            "Counting sort",
            Box::new(|numbers: &mut [i32]| algorithms::counting_sort::counting_sort(numbers)),
        ),
        (
            "Cycle sort",
            Box::new(|numbers: &mut [i32]| algorithms::cycle_sort::cycle_sort(numbers)),
        ),
        (
            "Gnome sort",
            Box::new(|numbers: &mut [i32]| algorithms::gnome_sort::gnome_sort(numbers)),
        ),
        (
            "Heap sort",
            Box::new(|numbers: &mut [i32]| algorithms::heap_sort::heap_sort(numbers)),
        ),
        (
            "Insertion sort",
            Box::new(|numbers: &mut [i32]| algorithms::insertion_sort::insertion_sort(numbers)),
        ),
        (
            "Library sort",
            Box::new(|numbers: &mut [i32]| algorithms::library_sort::library_sort(numbers)),
        ),
        (
            "Merge sort",
            Box::new(|numbers: &mut [i32]| algorithms::merge_sort::merge_sort(numbers)),
        ),
        (
            "Pancake sort",
            Box::new(|numbers: &mut [i32]| algorithms::pancake_sort::pancake_sort(numbers)),
        ),
        (
            "Patience sort",
            Box::new(|numbers: &mut [i32]| algorithms::patience_sort::patience_sort(numbers)),
        ),
        (
            "Quick sort",
            Box::new(|numbers: &mut [i32]| {
                algorithms::quick_sort::quick_sort(numbers, 0, numbers.len() - 1)
            }),
        ),
        (
            "Radix sort",
            Box::new(|numbers: &mut [i32]| algorithms::radix_sort::radix_sort(numbers)),
        ),
        (
            "Red-Black Tree sort",
            Box::new(|numbers: &mut [i32]| algorithms::rb_tree_sort::red_black_tree_sort(numbers)),
        ),
        (
            "Selection sort",
            Box::new(|numbers: &mut [i32]| algorithms::selection_sort::selection_sort(numbers)),
        ),
        (
            "Shell sort",
            Box::new(|numbers: &mut [i32]| algorithms::shell_sort::shell_sort(numbers)),
        ),
        (
            "Sleep sort",
            Box::new(|numbers: &mut [i32]| algorithms::sleep_sort::sleep_sort(numbers)),
        ),
        (
            "Smooth sort",
            Box::new(|numbers: &mut [i32]| algorithms::smooth_sort::smooth_sort(numbers)),
        ),
        (
            "Strand sort",
            Box::new(|numbers: &mut [i32]| algorithms::strand_sort::strand_sort(numbers)),
        ),
        (
            "Stooge sort",
            Box::new(|numbers: &mut [i32]| algorithms::stooge_sort::stooge_sort(numbers)),
        ),
        (
            "Tim sort",
            Box::new(|numbers: &mut [i32]| algorithms::tim_sort::tim_sort(numbers)),
        ),
        (
            "Tree sort",
            Box::new(|numbers: &mut [i32]| algorithms::tree_sort::tree_sort(numbers)),
        ),
    ];

    for (algorithm_name, sorting_algorithm) in sorting_algorithms {
        let mut numbers_to_sort: Vec<i32> = unsorted_numbers.clone();
        if algorithm_name == "Counting sort" || algorithm_name == "Radix sort" {
            if numbers_to_sort.iter().any(|&x| x < 0) {
                println!("{} is not intended for negative numbers", algorithm_name);
                continue;
            }
        }
        if (algorithm_name == "Bogo sort" || algorithm_name == "Sleep sort")
            && numbers_to_sort.len() > 10
        {
            println!("{algorithm_name} is too slow for more than 10 elements");
            continue;
        }
        if algorithm_name == "Sleep sort" {
            // If the numbers are too large, the program will take a long time to run
            if numbers_to_sort.iter().any(|&x| x > 10) {
                println!("{} is too slow for numbers greater than 10", algorithm_name);
                continue;
            }
        }
        let start_time: Instant = Instant::now();
        sorting_algorithm(&mut numbers_to_sort);
        let duration: std::time::Duration = start_time.elapsed();
        println!("{} took: {:?}", algorithm_name, duration);
    }
}

#[cfg(test)]
mod tests {
    use super::algorithms::*;

    fn test_sort<F>(sort: F)
    where
        F: Fn(&mut [i32]),
    {
        let mut numbers = [5, 2, 4, 1, 3];
        sort(&mut numbers);
        assert_eq!(numbers, [1, 2, 3, 4, 5]);
    }

    fn bitonic_sort_wrapper(array: &mut [i32]) {
        bitonic_sort::bitonic_sort(array, true);
    }

    #[test]
    fn test_bitonic_sort() {
        test_sort(bitonic_sort_wrapper);
    }

    #[test]
    fn test_bogo_sort() {
        test_sort(bogo_sort::bogo_sort);
    }

    #[test]
    fn test_bubble_sort() {
        test_sort(bubble_sort::bubble_sort);
    }

    #[test]
    fn test_cocktail_sort() {
        test_sort(cocktail_sort::cocktail_sort);
    }

    #[test]
    fn test_comb_sort() {
        test_sort(comb_sort::comb_sort);
    }

    #[test]
    fn test_counting_sort() {
        test_sort(counting_sort::counting_sort);
    }

    #[test]
    fn test_cycle_sort() {
        test_sort(cycle_sort::cycle_sort);
    }

    #[test]
    fn test_gnome_sort() {
        test_sort(gnome_sort::gnome_sort);
    }

    #[test]
    fn test_heap_sort() {
        test_sort(heap_sort::heap_sort);
    }

    #[test]
    fn test_insertion_sort() {
        test_sort(insertion_sort::insertion_sort);
    }

    #[test]
    fn test_library_sort() {
        test_sort(library_sort::library_sort);
    }

    #[test]
    fn test_merge_sort() {
        test_sort(merge_sort::merge_sort);
    }

    #[test]
    fn test_pancake_sort() {
        test_sort(pancake_sort::pancake_sort);
    }

    #[test]
    fn test_patience_sort() {
        test_sort(patience_sort::patience_sort);
    }

    #[test]
    fn test_quick_sort() {
        let mut numbers = [5, 2, 4, 1, 3];
        let len = numbers.len();
        quick_sort::quick_sort(&mut numbers, 0, len - 1);
        assert_eq!(numbers, [1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_radix_sort() {
        test_sort(radix_sort::radix_sort);
    }

    #[test]
    fn test_rb_tree_sort() {
        test_sort(rb_tree_sort::red_black_tree_sort);
    }

    #[test]
    fn test_selection_sort() {
        test_sort(selection_sort::selection_sort);
    }

    #[test]
    fn test_shell_sort() {
        test_sort(shell_sort::shell_sort);
    }

    #[test]
    fn test_sleep_sort() {
        let numbers = [5, 2, 4, 1, 3];
        
        // Skip the test if the array length is more than 10
        if numbers.len() > 10 {
            println!("Skipping sleep_sort test due to large array size.");
            return;
        }
    
        // Skip the test if any number is larger than 10
        if numbers.iter().any(|&x| x > 10) {
            println!("Skipping sleep_sort test due to large number in array.");
            return;
        }
    
        test_sort(sleep_sort::sleep_sort);
    }

    #[test]
    fn test_smooth_sort() {
        test_sort(smooth_sort::smooth_sort);
    }

    #[test]
    fn test_strand_sort() {
        test_sort(strand_sort::strand_sort);
    }

    #[test]
    fn test_stooge_sort() {
        test_sort(stooge_sort::stooge_sort);
    }

    #[test]
    fn test_tim_sort() {
        test_sort(tim_sort::tim_sort);
    }

    #[test]
    fn test_tree_sort() {
        test_sort(tree_sort::tree_sort);
    }
}
