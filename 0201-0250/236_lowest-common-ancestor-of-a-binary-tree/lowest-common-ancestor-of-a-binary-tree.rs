
use std::rc::Rc;
use std::cell::RefCell;

// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
// pub struct TreeNode {
//   pub val: i32,
//   pub left: Option<Rc<RefCell<TreeNode>>>,
//   pub right: Option<Rc<RefCell<TreeNode>>>,
// }
// 
// impl TreeNode {
//   #[inline]
//   pub fn new(val: i32) -> Self {
//     TreeNode {
//       val,
//       left: None,
//       right: None

//     }
//   }
// }

impl Solution {
    pub fn lowest_common_ancestor(root: Option<Rc<RefCell<TreeNode>>>, p: Option<Rc<RefCell<TreeNode>>>, q: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        if let Some(node) = root {
            let node_val = node.borrow().val;
            let p_val = p.as_ref().unwrap().borrow().val;
            let q_val = q.as_ref().unwrap().borrow().val;
            
            if node_val == p_val || node_val == q_val {
                return Some(node);
            }
            
            let left_lca = Solution::lowest_common_ancestor(node.borrow().left.clone(), p.clone(), q.clone());
            let right_lca = Solution::lowest_common_ancestor(node.borrow().right.clone(), p.clone(), q.clone());
            
            if left_lca.is_some() && right_lca.is_some() {
                return Some(node);
            } else if left_lca.is_none() && right_lca.is_some() {
                return right_lca;
            } else if left_lca.is_some() && right_lca.is_none() {
                return left_lca;
            } else {
                return None;
            }
        }
        
        None

    }
}
