
use std::rc::Rc;
use std::cell::RefCell;

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

impl Solution {
    pub fn deepest_leaves_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn dfs(node: &Option<Rc<RefCell<TreeNode>>>, depth: usize, depths: &mut Vec<(usize, i32)>) {
            if let Some(n) = node {
                let val = n.borrow().val;
                if depth >= depths.len() {
                    depths.push((depth, val));
                } else {
                    depths[depth].1 += val;
                }
                dfs(&n.borrow().left, depth + 1, depths);
                dfs(&n.borrow().right, depth + 1, depths);
            }
        }
        
        let mut depths = Vec::new();
        dfs(&root, 0, &mut depths);
        
        depths.iter().filter(|&&(d, _)| d == depths.last().unwrap().0).map(|&(_, v)| v).sum()
    }
}
