
use std::rc::Rc;
use std::cell::RefCell;

impl Solution {
    pub fn invert_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        if let Some(node) = root {
            let mut node = node.borrow_mut();
            let left = Self::invert_tree(node.left.take());
            let right = Self::invert_tree(node.right.take());
            let new_node = TreeNode {
                val: node.val,
                left: right,
                right: left,
            };
            Some(Rc::new(RefCell::new(new_node)))
        } else {
            None

        }
    }
}
