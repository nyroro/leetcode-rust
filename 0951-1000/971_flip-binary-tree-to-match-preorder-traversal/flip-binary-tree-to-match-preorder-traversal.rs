
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
    pub fn flip_match_voyage(root: Option<Rc<RefCell<TreeNode>>>, voyage: Vec<i32>) -> Vec<i32> {
        let mut result = Vec::new();

        fn dfs(node: Option<Rc<RefCell<TreeNode>>>, index: &mut usize, voyage: &Vec<i32>, result: &mut Vec<i32>) -> bool {
            if let Some(node) = node {
                let mut node = node.borrow_mut();
                if node.val != voyage[*index] {
                    return false;
                }
                *index += 1;
                if node.left.is_some() && node.left.as_ref().unwrap().borrow().val != voyage[*index] {
                    std::mem::swap(&mut node.left, &mut node.right);
                    result.push(node.val);
                }
                let left_result = dfs(node.left.take(), index, voyage, result);
                let right_result = dfs(node.right.take(), index, voyage, result);
                left_result && right_result

            } else {
                true

            }
        }

        let mut index = 0;
        if dfs(root, &mut index, &voyage, &mut result) && index == voyage.len() {
            result

        } else {
            vec![-1]
        }
    }
}
