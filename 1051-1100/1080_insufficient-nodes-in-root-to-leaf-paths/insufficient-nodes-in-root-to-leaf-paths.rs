
use std::rc::Rc;
use std::cell::RefCell;

// Definition for a binary tree node.
// pub struct TreeNode {
//   pub val: i32,
//   pub left: Option<Rc<RefCell<TreeNode>>>,
//   pub right: Option<Rc<RefCell<TreeNode>>>,
// }
// 
// impl TreeNode {
//   pub fn new(val: i32) -> Self {
//     TreeNode {
//       val,
//       left: None,
//       right: None

//     }
//   }
// }

impl Solution {
    pub fn sufficient_subset(root: Option<Rc<RefCell<TreeNode>>>, limit: i32) -> Option<Rc<RefCell<TreeNode>>> {
        fn helper(node: &Option<Rc<RefCell<TreeNode>>>, limit: i32, sum: i32) -> Option<Rc<RefCell<TreeNode>>> {
            if let Some(n) = node {
                let val = n.borrow().val;
                let left = n.borrow_mut().left.take();
                let right = n.borrow_mut().right.take();
                let new_sum = sum + val;
                if left.is_none() && right.is_none() {
                    if new_sum < limit {
                        return None;
                    } else {
                        return Some(Rc::clone(n));
                    }
                }
                n.borrow_mut().left = helper(&left, limit, new_sum);
                n.borrow_mut().right = helper(&right, limit, new_sum);
                if n.borrow().left.is_none() && n.borrow().right.is_none() {
                    return None;
                } else {
                    return Some(Rc::clone(n));
                }
            }
            None

        }
        
        helper(&root, limit, 0)
    }
}
