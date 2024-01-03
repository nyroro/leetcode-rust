
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
    pub fn longest_zig_zag(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut max_length = 0;
        Self::dfs(&root, true, 0, &mut max_length);
        Self::dfs(&root, false, 0, &mut max_length);
        max_length

    }
    
    fn dfs(node: &Option<Rc<RefCell<TreeNode>>>, is_left: bool, length: i32, max_length: &mut i32) {
        if let Some(n) = node {
            *max_length = (*max_length).max(length);
            let next_length = length + 1;
            if is_left {
                Self::dfs(&n.borrow().left, false, next_length, max_length);
                Self::dfs(&n.borrow().right, true, 1, max_length);
            } else {
                Self::dfs(&n.borrow().right, true, next_length, max_length);
                Self::dfs(&n.borrow().left, false, 1, max_length);
            }
        }
    }
}
