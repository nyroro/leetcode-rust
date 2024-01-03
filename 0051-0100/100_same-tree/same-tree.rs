
use std::rc::Rc;
use std::cell::RefCell;

impl Solution {
    pub fn is_same_tree(p: Option<Rc<RefCell<TreeNode>>>, q: Option<Rc<RefCell<TreeNode>>>) -> bool {
        match (p, q) {
            (None, None) => true,
            (Some(p), Some(q)) => {
                let p = p.borrow();
                let q = q.borrow();
                p.val == q.val && 
                Solution::is_same_tree(p.left.clone(), q.left.clone()) && 
                Solution::is_same_tree(p.right.clone(), q.right.clone())
            },
            _ => false,
        }
    }
}
