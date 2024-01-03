
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
use std::rc::Rc;
use std::cell::RefCell;

impl Solution {
    pub fn max_level_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        use std::collections::VecDeque;
        
        if root.is_none() {
            return 0;
        }
        
        let mut queue = VecDeque::new();
        queue.push_back(root);
        
        let mut max_sum = i32::MIN;
        let mut max_level = 0;
        let mut level = 1;
        
        while !queue.is_empty() {
            let mut level_sum = 0;
            let level_size = queue.len();
            
            for _ in 0..level_size {
                if let Some(node) = queue.pop_front().unwrap() {
                    let node = node.borrow();
                    level_sum += node.val;
                    
                    if let Some(left) = &node.left {
                        queue.push_back(Some(left.clone()));
                    }
                    
                    if let Some(right) = &node.right {
                        queue.push_back(Some(right.clone()));
                    }
                }
            }
            
            if level_sum > max_sum {
                max_sum = level_sum;
                max_level = level;
            }
            
            level += 1;
        }
        
        max_level

    }
}
