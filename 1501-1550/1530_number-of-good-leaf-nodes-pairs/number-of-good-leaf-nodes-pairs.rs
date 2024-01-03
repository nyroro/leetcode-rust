
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

use std::rc::Rc;
use std::cell::RefCell;

impl Solution {
    pub fn count_pairs(root: Option<Rc<RefCell<TreeNode>>>, distance: i32) -> i32 {
        // Helper function to calculate the distance between two nodes

        fn distance_between_nodes(node: Option<&Rc<RefCell<TreeNode>>>, distance: i32, result: &mut i32) -> Vec<i32> {
            // Base case: if the node is None or it's a leaf node

            if let Some(node) = node {
                let node = node.borrow();
                if node.left.is_none() && node.right.is_none() {
                    return vec![1];
                }
            } else {
                return vec![];
            }
            
            // Recursive case: calculate the distance between the current node and its children

            let mut distances = vec![];
            if let Some(node) = node {
                let node = node.borrow();
                let left_distances = distance_between_nodes(node.left.as_ref(), distance, result);
                let right_distances = distance_between_nodes(node.right.as_ref(), distance, result);
                
                for &left in &left_distances {
                    for &right in &right_distances {
                        if left + right <= distance {
                            *result += 1;
                        }
                    }
                }
                
                distances.extend(left_distances.iter().map(|x| x + 1));
                distances.extend(right_distances.iter().map(|x| x + 1));
            }
            
            distances

        }
        
        let mut result = 0;
        distance_between_nodes(root.as_ref(), distance, &mut result);
        result

    }
}
