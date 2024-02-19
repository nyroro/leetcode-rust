
use std::rc::Rc;
use std::cell::RefCell;

impl Solution {
    pub fn max_sum_bst(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut answer = i32::MIN;

        // Define the solve function

        fn solve(node: Option<Rc<RefCell<TreeNode>>>, answer: &mut i32) -> (i32, i32, i32) {
            match node {
                Some(inner) => {
                    let mut node = inner.borrow_mut();
                    let (left_min, left_max, left_sum) = solve(node.left.take(), answer);
                    let (right_min, right_max, right_sum) = solve(node.right.take(), answer);

                    if left_max >= node.val || right_min <= node.val {
                        return (i32::MIN, i32::MAX, 0);
                    }

                    let res = left_sum + right_sum + node.val;
                    *answer = (*answer).max(res); // Dereference the mutable reference and use max method


                    (left_min.min(node.val), right_max.max(node.val), res)
                }
                None => (i32::MAX, i32::MIN, 0)
            }
        }

        let _ = solve(root, &mut answer);
        if answer < 0 {
            return 0;
        }
        answer

    }
}
