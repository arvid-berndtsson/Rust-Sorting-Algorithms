use std::thread;
use std::time::Duration;

pub fn sleep_sort(unsorted_numbers: &mut [i32]) {
    let mut thread_handles = vec![];

    for &number in unsorted_numbers.iter() {
        let thread_handle = thread::spawn(move || {
            thread::sleep(Duration::from_secs(number.abs() as u64));
            number
        });
        thread_handles.push(thread_handle);
    }

    let mut index = 0;

    for handle in thread_handles {
        unsorted_numbers[index] = handle.join().unwrap();
        index += 1;
    }
}
