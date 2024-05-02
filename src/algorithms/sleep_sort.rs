use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

pub fn sleep_sort(unsorted_numbers: &mut [i32]) {
    let sorted_numbers = Arc::new(Mutex::new(Vec::new()));
    let mut thread_handles = vec![];

    for &number in unsorted_numbers.iter() {
        let sorted_numbers_clone = Arc::clone(&sorted_numbers);
        let thread_handle = thread::spawn(move || {
            thread::sleep(Duration::from_secs(number.abs() as u64));
            sorted_numbers_clone.lock().unwrap().push(number);
        });
        thread_handles.push(thread_handle);
    }

    for handle in thread_handles {
        handle.join().unwrap();
    }

    let sorted_numbers = Arc::try_unwrap(sorted_numbers)
        .unwrap()
        .into_inner()
        .unwrap();
    unsorted_numbers.copy_from_slice(&sorted_numbers);
}
