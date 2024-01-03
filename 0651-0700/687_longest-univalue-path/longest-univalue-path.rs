
use std::rc::Rc;
use std::cell::RefCell;

impl Solution {
    pub fn longest_univalue_path(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn helper(node: &Option<Rc<RefCell<TreeNode>>>, prev_val: i32, max_len: &mut i32) -> i32 {
            if let Some(inner) = node {
                let inner = inner.borrow();
                let left_len = helper(&inner.left, inner.val, max_len);
                let right_len = helper(&inner.right, inner.val, max_len);
                *max_len = (*max_len).max(left_len + right_len);
                if inner.val == prev_val {
                    return 1 + left_len.max(right_len);
                }
            }
            0

        }
        
        let mut max_len = 0;
        helper(&root, 0, &mut max_len);
        max_len

    }
}
