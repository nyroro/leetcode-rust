
use std::rc::Rc;
use std::cell::RefCell;

impl Solution {
    pub fn is_unival_tree(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        if let Some(node) = root {
            let val = node.borrow().val;
            let left = node.borrow().left.clone();
            let right = node.borrow().right.clone();
            Self::is_unival_tree(left.clone())
                && Self::is_unival_tree(right.clone())
                && left.map_or(true, |n| n.borrow().val == val)
                && right.map_or(true, |n| n.borrow().val == val)
        } else {
            true

        }
    }
}
