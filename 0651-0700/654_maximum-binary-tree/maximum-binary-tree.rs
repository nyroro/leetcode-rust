
use std::rc::Rc;
use std::cell::RefCell;

impl Solution {
    pub fn construct_maximum_binary_tree(nums: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        // Define a recursive function to construct the maximum binary tree

        fn construct_tree(nums: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
            if nums.is_empty() {
                return None; // Return None for an empty array

            }
            
            let mut max_index = 0;
            for i in 1..nums.len() {
                if nums[i] > nums[max_index] {
                    max_index = i; // Find the index of the maximum value in the array

                }
            }
            
            let root = Rc::new(RefCell::new(TreeNode::new(nums[max_index]))); // Create a new TreeNode with the maximum value as the root
            
            // Recursively build the left and right subtrees

            root.borrow_mut().left = construct_tree(&nums[0..max_index]);
            root.borrow_mut().right = construct_tree(&nums[max_index + 1..nums.len()]);
            
            Some(root) // Return the constructed tree

        }
        
        construct_tree(&nums) // Call the recursive function with the input array

    }
}
