pub fn heap_sort(array: &mut [i32]) {
  let array_length = array.len();
  for index in (0..array_length / 2).rev() {
      heapify(array, array_length, index);
  }
  for index in (0..array_length).rev() {
      array.swap(0, index);
      heapify(array, index, 0);
  }
}

fn heapify(array: &mut [i32], heap_size: usize, root_index: usize) {
  let mut largest = root_index;
  let left_child = 2 * root_index + 1;
  let right_child = 2 * root_index + 2;

  if left_child < heap_size && array[left_child] > array[largest] {
      largest = left_child;
  }
  if right_child < heap_size && array[right_child] > array[largest] {
      largest = right_child;
  }
  if largest != root_index {
      array.swap(root_index, largest);
      heapify(array, heap_size, largest);
  }
}