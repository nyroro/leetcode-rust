
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
    pub fn binary_tree_paths(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<String> {
        let mut result: Vec<String> = Vec::new();
        let path: String = String::new();
        Self::dfs(&root, &path, &mut result);
        result

    }
    
    fn dfs(node: &Option<Rc<RefCell<TreeNode>>>, path: &String, result: &mut Vec<String>) {
        if let Some(n) = node {
            let mut path = path.clone();
            path.push_str(&n.borrow().val.to_string());
            
            if n.borrow().left.is_none() && n.borrow().right.is_none() {
                result.push(path);
            } else {
                path.push_str("->");
                Self::dfs(&n.borrow().left, &path, result);
                Self::dfs(&n.borrow().right, &path, result);
            }
        }
    }
}
