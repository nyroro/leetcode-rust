
use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;

impl Solution {
    pub fn path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> i32 {
        let mut prefix_sum_count: HashMap<i64, i32> = HashMap::new();
        prefix_sum_count.insert(0, 1); // Initialize with 0 prefix sum count


        // Recursive function to calculate the count of paths

        fn get_path_count(node: &Option<Rc<RefCell<TreeNode>>>, target: i32, prefix_sum: i64, prefix_sum_count: &mut HashMap<i64, i32>) -> i32 {
            if let Some(n) = node {
                let val = n.borrow().val;
                let mut count = 0;
                let current_prefix_sum = prefix_sum + val as i64;

                // Calculate the count of paths for the current prefix sum

                count += *prefix_sum_count.get(&(current_prefix_sum - target as i64)).unwrap_or(&0);

                // Update the prefix sum count

                *prefix_sum_count.entry(current_prefix_sum).or_insert(0) += 1;

                // Recursively calculate the count of paths for left and right subtrees

                count += get_path_count(&n.borrow().left, target, current_prefix_sum, prefix_sum_count);
                count += get_path_count(&n.borrow().right, target, current_prefix_sum, prefix_sum_count);

                // Decrement the frequency of the current prefix sum (backtrack)
                *prefix_sum_count.get_mut(&current_prefix_sum).unwrap() -= 1;

                count

            } else {
                0

            }
        }

        // Call the recursive function to get the path count

        get_path_count(&root, target_sum, 0, &mut prefix_sum_count)
    }
}
