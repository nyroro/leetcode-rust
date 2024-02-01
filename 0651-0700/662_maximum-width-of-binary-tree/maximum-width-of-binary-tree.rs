
use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;
use std::collections::VecDeque;

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
    pub fn width_of_binary_tree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut max_width = 0;
        let mut queue = VecDeque::new();
        let mut map = HashMap::new();
        
        queue.push_back((root.clone(), 0, 0));
        
        while !queue.is_empty() {
            if let Some((node, depth, pos)) = queue.pop_front() {
                if !map.contains_key(&depth) {
                    map.insert(depth, pos);
                }
                max_width = max_width.max(pos - map[&depth] + 1);
                
                if let Some(left) = node.as_ref().and_then(|n| n.borrow().left.clone()) {
                    queue.push_back((left, depth + 1, pos * 2));
                }
                if let Some(right) = node.as_ref().and_then(|n| n.borrow().right.clone()) {
                    queue.push_back((right, depth + 1, pos * 2 + 1));
                }
            }
        }
        
        max_width as i32

    }
}
