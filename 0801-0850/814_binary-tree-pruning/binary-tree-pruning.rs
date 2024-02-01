
use std::rc::Rc;
use std::cell::RefCell;

impl Solution {
    pub fn prune_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        if let Some(node) = root {
            let mut node = node.borrow_mut();
            node.left = Solution::prune_tree(node.left.take());
            node.right = Solution::prune_tree(node.right.take());
            if node.val == 0 && node.left.is_none() && node.right.is_none() {
                return None;
            }
        }
        root

    }
}
