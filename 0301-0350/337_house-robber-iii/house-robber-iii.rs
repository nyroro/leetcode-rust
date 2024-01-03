
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
    pub fn rob(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn dfs(node: &Option<Rc<RefCell<TreeNode>>>) -> (i32, i32) {
            if let Some(n) = node {
                let n = n.borrow();
                let (left_rob, left_not_rob) = dfs(&n.left);
                let (right_rob, right_not_rob) = dfs(&n.right);
                let rob = n.val + left_not_rob + right_not_rob;
                let not_rob = left_rob.max(left_not_rob) + right_rob.max(right_not_rob);
                (rob, not_rob)
            } else {
                (0, 0)
            }
        }
        
        let (rob, not_rob) = dfs(&root);
        rob.max(not_rob)
    }
}
