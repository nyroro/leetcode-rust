
use std::rc::Rc;
use std::cell::RefCell;

impl Solution {
    pub fn remove_leaf_nodes(root: Option<Rc<RefCell<TreeNode>>>, target: i32) -> Option<Rc<RefCell<TreeNode>>> {
        if let Some(node) = root {
            let mut node = node.borrow_mut();
            node.left = Solution::remove_leaf_nodes(node.left.take(), target);
            node.right = Solution::remove_leaf_nodes(node.right.take(), target);
            if node.left.is_none() && node.right.is_none() && node.val == target {
                return None;
            }
        }
        root

    }
}
