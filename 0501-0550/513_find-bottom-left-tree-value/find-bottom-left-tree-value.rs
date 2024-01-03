
use std::rc::Rc;
use std::cell::RefCell;
use std::collections::VecDeque;

impl Solution {
    pub fn find_bottom_left_value(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut queue = VecDeque::new();
        if let Some(node) = root {
            queue.push_back(node);
        }
        
        let mut result = 0;
        
        while !queue.is_empty() {
            let size = queue.len();
            for i in 0..size {
                if let Some(node) = queue.pop_front() {
                    let node = node.borrow();
                    if i == 0 {
                        result = node.val;
                    }
                    if let Some(left) = &node.left {
                        queue.push_back(left.clone());
                    }
                    if let Some(right) = &node.right {
                        queue.push_back(right.clone());
                    }
                }
            }
        }
        
        result

    }
}
