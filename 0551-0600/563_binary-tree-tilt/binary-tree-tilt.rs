
use std::rc::Rc;
use std::cell::RefCell;

// pub struct TreeNode {
//   pub val: i32,
//   pub left: Option<Rc<RefCell<TreeNode>>>,
//   pub right: Option<Rc<RefCell<TreeNode>>>,
// }

impl Solution {
    pub fn find_tilt(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn tilt_sum(node: &Option<Rc<RefCell<TreeNode>>>, sum: &mut i32) -> i32 {
            if let Some(n) = node {
                let n = n.borrow();
                let left_sum = tilt_sum(&n.left, sum);
                let right_sum = tilt_sum(&n.right, sum);
                *sum += (left_sum - right_sum).abs();
                n.val + left_sum + right_sum

            } else {
                0

            }
        }
        
        let mut result = 0;
        tilt_sum(&root, &mut result);
        result

    }
}
