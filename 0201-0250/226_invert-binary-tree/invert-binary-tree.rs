
use std::rc::Rc;
use std::cell::RefCell;

impl Solution {
    pub fn invert_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        if let Some(node) = root {
            let mut node = node.borrow_mut();
            let left = Self::invert_tree(node.left.take());
            let right = Self::invert_tree(node.right.take());
            node.left = right;
            node.right = left;
            Some(node.clone())
        } else {
            None

        }
    }
}
