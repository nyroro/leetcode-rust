
use std::rc::Rc;
use std::cell::RefCell;

impl Solution {
    pub fn lowest_common_ancestor(root: Option<Rc<RefCell<TreeNode>>>, p: Option<Rc<RefCell<TreeNode>>>, q: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        fn find_lca(node: Option<&Rc<RefCell<TreeNode>>>, p_val: i32, q_val: i32) -> Option<Rc<RefCell<TreeNode>>> {
            if let Some(n) = node {
                let val = n.borrow().val;
                if val > p_val && val > q_val {
                    return find_lca(n.borrow().left.as_ref(), p_val, q_val);
                } else if val < p_val && val < q_val {
                    return find_lca(n.borrow().right.as_ref(), p_val, q_val);
                } else {
                    return Some(n.clone());
                }
            }
            None

        }

        let p_val = p.as_ref().unwrap().borrow().val;
        let q_val = q.as_ref().unwrap().borrow().val;
        find_lca(root.as_ref(), p_val, q_val)
    }
}
