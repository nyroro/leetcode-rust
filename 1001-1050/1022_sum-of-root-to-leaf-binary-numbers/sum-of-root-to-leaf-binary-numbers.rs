
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
    pub fn sum_root_to_leaf(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn dfs(node: &Option<Rc<RefCell<TreeNode>>>, sum: i32) -> i32 {
            if let Some(n) = node {
                let n = n.borrow();
                let sum = (sum << 1) + n.val;
                if n.left.is_none() && n.right.is_none() {
                    return sum;
                }
                return dfs(&n.left, sum) + dfs(&n.right, sum);
            }
            0

        }
        
        dfs(&root, 0)
    }
}
