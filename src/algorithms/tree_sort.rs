pub struct TreeNode<T> {
    node_value: T,
    left_child: Option<Box<TreeNode<T>>>,
    right_child: Option<Box<TreeNode<T>>>,
}

impl<T: Ord> TreeNode<T> {
    pub fn insert_value(&mut self, new_value: T) {
        if new_value <= self.node_value {
            if let Some(left_child) = &mut self.left_child {
                left_child.insert_value(new_value);
            } else {
                self.left_child = Some(Box::new(TreeNode {
                    node_value: new_value,
                    left_child: None,
                    right_child: None,
                }));
            }
        } else {
            if let Some(right_child) = &mut self.right_child {
                right_child.insert_value(new_value);
            } else {
                self.right_child = Some(Box::new(TreeNode {
                    node_value: new_value,
                    left_child: None,
                    right_child: None,
                }));
            }
        }
    }

    pub fn in_order_traversal(&self, sorted_array: &mut Vec<T>)
    where
        T: Clone,
    {
        if let Some(left_child) = &self.left_child {
            left_child.in_order_traversal(sorted_array);
        }
        sorted_array.push(self.node_value.clone());
        if let Some(right_child) = &self.right_child {
            right_child.in_order_traversal(sorted_array);
        }
    }
}

pub fn tree_sort<T: Ord + Copy>(unsorted_array: &mut [T]) {
    if let Some((first_element, remaining_elements)) = unsorted_array.split_first() {
        let mut root_node = TreeNode {
            node_value: *first_element,
            left_child: None,
            right_child: None,
        };
        for value in remaining_elements.iter() {
            root_node.insert_value(*value);
        }
        let mut sorted_array = Vec::new();
        root_node.in_order_traversal(&mut sorted_array);
        unsorted_array.copy_from_slice(&sorted_array);
    }
}
