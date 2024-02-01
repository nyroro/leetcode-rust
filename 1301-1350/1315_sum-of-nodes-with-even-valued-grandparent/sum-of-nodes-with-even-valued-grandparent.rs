
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
    pub fn sum_even_grandparent(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn dfs(node: &Option<Rc<RefCell<TreeNode>>>, parent: Option<i32>, grandparent: Option<i32>) -> i32 {
            if let Some(n) = node {
                let n = n.borrow();
                let mut sum = 0;
                if let Some(grandparent_val) = grandparent {
                    if grandparent_val % 2 == 0 {
                        sum += n.val;
                    }
                }
                sum += dfs(&n.left, Some(n.val), parent);
                sum += dfs(&n.right, Some(n.val), parent);
                sum

            } else {
                0

            }
        }
        
        dfs(&root, None, None)
    }
}
