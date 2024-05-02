extern crate rand;
use rand::seq::SliceRandom;
use rand::thread_rng;

/// Bogo Sort is a highly inefficient sorting algorithm based on the generate and test paradigm.
/// The algorithm successively generates permutations of its input until it finds one that is sorted.
///
/// # Examples
///
/// ```
/// let mut numbers = vec![4, 3, 2, 1];
/// bogo_sort(&mut numbers);
/// assert_eq!(numbers, vec![1, 2, 3, 4]);
/// ```
///
/// # Panics
///
/// No panics occur in this function.
///
/// # Safety
///
/// This function does not use any unsafe blocks.
///
/// # Performance
///
/// Bogo Sort has a worst-case and average time complexity of O((n+1)!), where n is the number of elements in the input.
/// This makes it impractical for sorting large arrays. Bogo Sort is not used for any serious computational tasks. It is primarily a joke or pedagogical algorithm.
pub fn bogo_sort<T: Ord>(elements: &mut [T]) {
    while !is_sorted(elements) {
        elements.shuffle(&mut thread_rng());
    }
}

fn is_sorted<T: Ord>(elements: &[T]) -> bool {
    for index in 0..elements.len() - 1 {
        if elements[index] > elements[index + 1] {
            return false;
        }
    }
    true
}
