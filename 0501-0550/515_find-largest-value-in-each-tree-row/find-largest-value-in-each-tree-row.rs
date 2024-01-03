
use std::rc::Rc;
use std::cell::RefCell;
use std::collections::VecDeque;

impl Solution {
    pub fn largest_values(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut result = Vec::new();
        if let Some(node) = root {
            let mut queue = VecDeque::new();
            queue.push_back(node);
            while !queue.is_empty() {
                let mut max_val = i32::min_value();
                for _ in 0..queue.len() {
                    if let Some(curr) = queue.pop_front() {
                        let curr_val = curr.borrow().val;
                        max_val = max_val.max(curr_val);
                        if let Some(left) = curr.borrow().left.clone() {
                            queue.push_back(left);
                        }
                        if let Some(right) = curr.borrow().right.clone() {
                            queue.push_back(right);
                        }
                    }
                }
                result.push(max_val);
            }
        }
        result

    }
}
