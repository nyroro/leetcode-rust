
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
    pub fn is_complete_tree(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        if let Some(root) = root {
            let mut queue = std::collections::VecDeque::new();
            queue.push_back(root);

            let mut is_last_level = false;

            while let Some(node) = queue.pop_front() {
                let node = node.borrow();

                if node.left.is_none() {
                    is_last_level = true;
                } else {
                    if is_last_level {
                        return false;
                    }
                    queue.push_back(node.left.clone().unwrap());
                }

                if node.right.is_none() {
                    is_last_level = true;
                } else {
                    if is_last_level {
                        return false;
                    }
                    queue.push_back(node.right.clone().unwrap());
                }
            }
        }

        true

    }
}
