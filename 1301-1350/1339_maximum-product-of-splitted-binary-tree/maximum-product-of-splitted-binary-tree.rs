
use std::rc::Rc;
use std::cell::RefCell;

// Definition for a binary tree node.
// pub struct TreeNode {
//   pub val: i32,
//   pub left: Option<Rc<RefCell<TreeNode>>>,
//   pub right: Option<Rc<RefCell<TreeNode>>>,
// }
// 
// impl TreeNode {
//   pub fn new(val: i32) -> Self {
//     TreeNode {
//       val,
//       left: None,
//       right: None

//     }
//   }
// }



impl Solution {
    pub fn max_product(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn dfs(node: &Option<Rc<RefCell<TreeNode>>>, total_sum: i64, max_product: &mut i64) -> i64 {
            if let Some(n) = node {
                let n = n.borrow();
                let left_sum = dfs(&n.left, total_sum, max_product);
                let right_sum = dfs(&n.right, total_sum, max_product);
                let current_sum = n.val as i64 + left_sum + right_sum;
                *max_product = (*max_product).max((total_sum - current_sum) * current_sum);
                return current_sum;
            }
            0

        }

        let mut max_product = 0;
        let total_sum = Self::calculate_total_sum(&root);
        let _ = dfs(&root, total_sum, &mut max_product);
        (max_product % (10_i64.pow(9) + 7)) as i32

    }

    fn calculate_total_sum(node: &Option<Rc<RefCell<TreeNode>>>) -> i64 {
        if let Some(n) = node {
            let n = n.borrow();
            return n.val as i64 + Self::calculate_total_sum(&n.left) + Self::calculate_total_sum(&n.right);
        }
        0

    }
}
