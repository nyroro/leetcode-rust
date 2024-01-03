
use std::rc::Rc;
use std::cell::RefCell;
use std::collections::VecDeque;

impl Solution {
    pub fn average_of_levels(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<f64> {
        let mut result = Vec::new();
        if let Some(node) = root {
            let mut queue = VecDeque::new();
            queue.push_back(node);

            while !queue.is_empty() {
                let mut sum = 0i64;
                let mut count = 0;
                let level_size = queue.len();

                for _ in 0..level_size {
                    if let Some(node) = queue.pop_front() {
                        let node = node.borrow();
                        sum += node.val as i64;
                        count += 1;

                        if let Some(left) = &node.left {
                            queue.push_back(left.clone());
                        }
                        if let Some(right) = &node.right {
                            queue.push_back(right.clone());
                        }
                    }
                }

                result.push(sum as f64 / count as f64);
            }
        }

        result

    }
}
