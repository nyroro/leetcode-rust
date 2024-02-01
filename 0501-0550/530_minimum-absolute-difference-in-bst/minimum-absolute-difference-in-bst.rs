
use std::rc::Rc;
use std::cell::RefCell;

impl Solution {
    pub fn get_minimum_difference(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut min_abs_diff = i32::max_value();
        let mut prev_val: Option<i32> = None;
        
        Self::inorder_traversal(&root, &mut prev_val, &mut min_abs_diff);
        
        min_abs_diff

    }
    
    fn inorder_traversal(node: &Option<Rc<RefCell<TreeNode>>>, prev_val: &mut Option<i32>, min_abs_diff: &mut i32) {
        if let Some(n) = node {
            let n = n.borrow();
            Self::inorder_traversal(&n.left, prev_val, min_abs_diff);
            
            if let Some(prev) = prev_val {
                *min_abs_diff = (*min_abs_diff).min(n.val - *prev);
            }
            *prev_val = Some(n.val);
            
            Self::inorder_traversal(&n.right, prev_val, min_abs_diff);
        }
    }
}
