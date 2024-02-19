
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
    pub fn closest_nodes(root: Option<Rc<RefCell<TreeNode>>>, queries: Vec<i32>) -> Vec<Vec<i32>> {
        // Helper function to perform DFS and create a sorted list of node values

        fn dfs(root: &Option<Rc<RefCell<TreeNode>>>, bv: &mut Vec<i32>) {
            if let Some(node) = root {
                let node_ref = node.borrow();
                dfs(&node_ref.left, bv);
                bv.push(node_ref.val);
                dfs(&node_ref.right, bv);
            }
        }

        // Helper function to find the minimum value in the sorted list

        fn min1(bv: &Vec<i32>, val: i32) -> i32 {
            let mut ans = -1;
            let mut i = 0;
            let mut j = bv.len() as i32 - 1;
            while i <= j {
                let mid = i + (j - i) / 2;
                if val == bv[mid as usize] {
                    return val;
                }
                if val > bv[mid as usize] {
                    ans = bv[mid as usize];
                    i = mid + 1;
                } else {
                    j = mid - 1;
                }
            }
            ans

        }

        // Helper function to find the maximum value in the sorted list

        fn max1(bv: &Vec<i32>, val: i32) -> i32 {
            let mut ans = -1;
            let mut i = 0;
            let mut j = bv.len() as i32 - 1;
            while i <= j {
                let mid = i + (j - i) / 2;
                if val == bv[mid as usize] {
                    return val;
                }
                if val < bv[mid as usize] {
                    ans = bv[mid as usize];
                    j = mid - 1;
                } else {
                    i = mid + 1;
                }
            }
            ans

        }

        // Main function to find the closest nodes for each query

        let mut result = Vec::new();
        if let Some(node) = root {
            let mut bv = Vec::new();
            dfs(&Some(node), &mut bv);
            for q in queries {
                let lb = min1(&bv, q);
                let ub = max1(&bv, q);
                result.push(vec![lb, ub]);
            }
        }
        result

    }
}
