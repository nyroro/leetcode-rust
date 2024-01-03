
use std::rc::Rc;
use std::cell::RefCell;

impl Solution {
    pub fn is_symmetric(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        fn is_mirror(left: Option<Rc<RefCell<TreeNode>>>, right: Option<Rc<RefCell<TreeNode>>>) -> bool {
            match (left, right) {
                (None, None) => true,
                (Some(l), Some(r)) => {
                    let l = l.borrow();
                    let r = r.borrow();
                    l.val == r.val && is_mirror(l.left.clone(), r.right.clone()) && is_mirror(l.right.clone(), r.left.clone())
                }
                _ => false,
            }
        }
        
        match root {
            None => true,
            Some(root) => {
                let root = root.borrow();
                is_mirror(root.left.clone(), root.right.clone())
            }
        }
    }
}
