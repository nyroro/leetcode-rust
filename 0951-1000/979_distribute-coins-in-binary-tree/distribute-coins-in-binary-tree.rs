
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
    pub fn distribute_coins(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn dfs(node: &Option<Rc<RefCell<TreeNode>>>, moves: &mut i32) -> i32 {
            if let Some(n) = node {
                let n = n.borrow();
                let left_excess = dfs(&n.left, moves);
                let right_excess = dfs(&n.right, moves);
                *moves += left_excess.abs() + right_excess.abs();
                n.val + left_excess + right_excess - 1

            } else {
                0

            }
        }
        
        let mut moves = 0;
        dfs(&root, &mut moves);
        moves

    }
}
