
use std::rc::Rc;
use std::cell::RefCell;

impl Solution {
    pub fn check_tree(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        if let Some(node) = root {
            let node = node.borrow();
            let mut sum = node.val;
            if let Some(left) = &node.left {
                sum -= left.borrow().val;
            }
            if let Some(right) = &node.right {
                sum -= right.borrow().val;
            }
            sum == 0

        } else {
            false

        }
    }
}
