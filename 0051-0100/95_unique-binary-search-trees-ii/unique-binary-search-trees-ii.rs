
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
    pub fn generate_trees(n: i32) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
        fn generate_trees_helper(start: i32, end: i32) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
            let mut trees = Vec::new();
            if start > end {
                trees.push(None);
                return trees;
            }
            for i in start..=end {
                let left_trees = generate_trees_helper(start, i - 1);
                let right_trees = generate_trees_helper(i + 1, end);
                for left in &left_trees {
                    for right in &right_trees {
                        let root = Some(Rc::new(RefCell::new(TreeNode {
                            val: i,
                            left: left.clone(),
                            right: right.clone(),
                        })));
                        trees.push(root);
                    }
                }
            }
            trees

        }
        generate_trees_helper(1, n)
    }
}
