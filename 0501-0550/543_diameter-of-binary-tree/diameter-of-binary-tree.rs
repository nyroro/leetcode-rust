
use std::rc::Rc;
use std::cell::RefCell;

impl Solution {
    pub fn diameter_of_binary_tree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn depth(node: &Option<Rc<RefCell<TreeNode>>>, diameter: &mut i32) -> i32 {
            if let Some(n) = node {
                let n = n.borrow();
                let left_depth = depth(&n.left, diameter);
                let right_depth = depth(&n.right, diameter);
                *diameter = (*diameter).max(left_depth + right_depth);
                1 + left_depth.max(right_depth)
            } else {
                0

            }
        }
        
        let mut diameter = 0;
        depth(&root, &mut diameter);
        diameter

    }
}
