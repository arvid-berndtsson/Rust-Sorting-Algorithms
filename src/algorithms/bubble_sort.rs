pub fn bubble_sort(array: &mut [i32]) {
  let array_length = array.len();
  for _ in 0..array_length {
      for index in 0..array_length - 1 {
          if array[index] > array[index + 1] {
              array.swap(index, index + 1);
          }
      }
  }
}