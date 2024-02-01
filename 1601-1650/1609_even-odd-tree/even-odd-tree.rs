
use std::collections::VecDeque;
use std::rc::Rc;
use std::cell::RefCell;



impl Solution {
    pub fn is_even_odd_tree(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        if let Some(root) = root {
            let mut queue = VecDeque::new();
            queue.push_back(root);
            let mut even = true;
            
            while !queue.is_empty() {
                let mut prev = if even { 0 } else { std::i32::MAX };
                let size = queue.len();
                
                for _ in 0..size {
                    if let Some(node) = queue.pop_front() {
                        let value = node.borrow().val;
                        
                        if even {
                            if value % 2 == 0 || value <= prev {
                                return false;
                            }
                        } else {
                            if value % 2 != 0 || value >= prev {
                                return false;
                            }
                        }
                        
                        prev = value;
                        
                        if let Some(left) = &node.borrow().left {
                            queue.push_back(left.clone());
                        }
                        
                        if let Some(right) = &node.borrow().right {
                            queue.push_back(right.clone());
                        }
                    }
                }
                
                even = !even;
            }
        }
        
        true

    }
}
