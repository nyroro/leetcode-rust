
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
    pub fn max_product(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn sum_and_product(node: &Option<Rc<RefCell<TreeNode>>>, sums: &mut Vec<i64>, total_sum: &mut i64) -> i64 {
            if let Some(n) = node {
                let n = n.borrow();
                let left_sum = sum_and_product(&n.left, sums, total_sum);
                let right_sum = sum_and_product(&n.right, sums, total_sum);
                let current_sum = n.val as i64 + left_sum + right_sum;
                sums.push(current_sum);
                *total_sum += current_sum;
                return current_sum;
            }
            0

        }

        let mut sums = Vec::new();
        let mut total_sum = 0;
        let _ = sum_and_product(&root, &mut sums, &mut total_sum);
        let mut max_product = 0;
        for sum in sums {
            let product = sum * (total_sum - sum);
            max_product = max_product.max(product);
        }
        (max_product % (10_i64.pow(9) + 7)) as i32

    }
}
