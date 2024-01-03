
use std::rc::Rc;
use std::cell::RefCell;

impl Solution {
    pub fn min_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if let Some(node) = root {
            let left_depth = Self::min_depth(node.borrow().left.clone());
            let right_depth = Self::min_depth(node.borrow().right.clone());
            
            if left_depth == 0 || right_depth == 0 {
                return 1 + left_depth.max(right_depth);
            } else {
                return 1 + left_depth.min(right_depth);
            }
        } else {
            return 0;
        }
    }
}
