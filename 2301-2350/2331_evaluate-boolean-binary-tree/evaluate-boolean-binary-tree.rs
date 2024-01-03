
use std::rc::Rc;
use std::cell::RefCell;

// Definition for a binary tree node.
// pub struct TreeNode {
//   pub val: i32,
//   pub left: Option<Rc<RefCell<TreeNode>>>,
//   pub right: Option<Rc<RefCell<TreeNode>>>,
// }

impl Solution {
    pub fn evaluate_tree(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        if let Some(node) = root {
            let node_ref = node.borrow();
            if node_ref.left.is_none() && node_ref.right.is_none() {
                return node_ref.val == 1;
            } else {
                let left_val = Solution::evaluate_tree(node_ref.left.clone());
                let right_val = Solution::evaluate_tree(node_ref.right.clone());
                match node_ref.val {
                    2 => left_val || right_val,
                    3 => left_val && right_val,
                    _ => false,
                }
            }
        } else {
            false

        }
    }
}
