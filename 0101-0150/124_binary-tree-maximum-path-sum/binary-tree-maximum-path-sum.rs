
use std::rc::Rc;
use std::cell::RefCell;

// // Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
// pub struct TreeNode {
//   pub val: i32,
//   pub left: Option<Rc<RefCell<TreeNode>>>,
//   pub right: Option<Rc<RefCell<TreeNode>>>,
// }
// 
// impl TreeNode {
//   #[inline]
//   pub fn new(val: i32) -> Self {
//     TreeNode {
//       val,
//       left: None,
//       right: None

//     }
//   }
// }

impl Solution {
    pub fn max_path_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut max_path = i32::MIN;
        Self::max_path_sum_helper(&root, &mut max_path);
        max_path

    }
    
    fn max_path_sum_helper(root: &Option<Rc<RefCell<TreeNode>>>, max_path: &mut i32) -> i32 {
        if let Some(node) = root {
            let node = node.borrow();
            let left_sum = Self::max_path_sum_helper(&node.left, max_path).max(0);
            let right_sum = Self::max_path_sum_helper(&node.right, max_path).max(0);
            let max_sum = node.val + left_sum + right_sum;
            *max_path = (*max_path).max(max_sum);
            node.val + left_sum.max(right_sum)
        } else {
            0

        }
    }
}
