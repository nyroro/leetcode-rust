
use std::rc::Rc;
use std::cell::RefCell;
use std::collections::VecDeque;

impl Solution {
    pub fn right_side_view(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut result = Vec::new();
        if let Some(root) = root {
            let mut queue = VecDeque::new();
            queue.push_back(root);
            
            while !queue.is_empty() {
                let size = queue.len();
                
                for i in 0..size {
                    let node = queue.pop_front().unwrap();
                    let node = node.borrow();
                    
                    if i == size - 1 {
                        result.push(node.val);
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
