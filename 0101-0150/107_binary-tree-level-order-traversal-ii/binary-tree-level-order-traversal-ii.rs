
use std::rc::Rc;
use std::cell::RefCell;

// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
  pub val: i32,
  pub left: Option<Rc<RefCell<TreeNode>>>,
  pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
  #[inline]
  pub fn new(val: i32) -> Self {
    TreeNode {
      val,
      left: None,
      right: None

    }
  }
}

impl Solution {
    pub fn level_order_bottom(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let mut result: Vec<Vec<i32>> = Vec::new();
        if let Some(root) = root {
            let mut queue = std::collections::VecDeque::new();
            queue.push_back(root);
            
            while !queue.is_empty() {
                let size = queue.len();
                let mut level: Vec<i32> = Vec::new();
                
                for _ in 0..size {
                    if let Some(node) = queue.pop_front() {
                        let node = node.borrow();
                        level.push(node.val);
                        
                        if let Some(left) = &node.left {
                            queue.push_back(left.clone());
                        }
                        
                        if let Some(right) = &node.right {
                            queue.push_back(right.clone());
                        }
                    }
                }
                
                result.insert(0, level);
            }
        }
        
        result

    }
}
