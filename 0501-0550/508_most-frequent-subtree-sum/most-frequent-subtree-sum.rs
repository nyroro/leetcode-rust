
use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;

// Definition for a binary tree node.
// pub struct TreeNode {
//   pub val: i32,
//   pub left: Option<Rc<RefCell<TreeNode>>>,
//   pub right: Option<Rc<RefCell<TreeNode>>>,
// }

impl Solution {
    pub fn find_frequent_tree_sum(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut sum_freq: HashMap<i32, i32> = HashMap::new();
        let mut max_freq = 0;

        Self::dfs(&root, &mut sum_freq, &mut max_freq);

        let mut result: Vec<i32> = Vec::new();
        for (&sum, &freq) in &sum_freq {
            if freq == max_freq {
                result.push(sum);
            }
        }

        result

    }

    fn dfs(root: &Option<Rc<RefCell<TreeNode>>>, sum_freq: &mut HashMap<i32, i32>, max_freq: &mut i32) -> i32 {
        if let Some(node) = root {
            let node_borrow = node.borrow();
            let left_sum = Self::dfs(&node_borrow.left, sum_freq, max_freq);
            let right_sum = Self::dfs(&node_borrow.right, sum_freq, max_freq);
            let subtree_sum = node_borrow.val + left_sum + right_sum;
            let count = sum_freq.entry(subtree_sum).or_insert(0);
            *count += 1;
            *max_freq = (*max_freq).max(*count);
            subtree_sum

        } else {
            0

        }
    }
}
