
use std::rc::Rc;
use std::cell::RefCell;

impl Solution {
    pub fn is_balanced(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        fn height(node: Option<&Rc<RefCell<TreeNode>>>) -> i32 {
            if let Some(node) = node {
                let node = node.borrow();
                let left_height = height(node.left.as_ref());
                let right_height = height(node.right.as_ref());
                1 + left_height.max(right_height)
            } else {
                0

            }
        }
        
        fn is_balanced_helper(node: Option<&Rc<RefCell<TreeNode>>>) -> bool {
            if let Some(node) = node {
                let node = node.borrow();
                let left_height = height(node.left.as_ref());
                let right_height = height(node.right.as_ref());
                let height_diff = (left_height - right_height).abs();
                height_diff <= 1 && is_balanced_helper(node.left.as_ref()) && is_balanced_helper(node.right.as_ref())
            } else {
                true

            }
        }
        
        is_balanced_helper(root.as_ref())
    }
}
