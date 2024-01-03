
use std::rc::Rc;
use std::cell::RefCell;

impl Solution {
    pub fn zigzag_level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let mut result: Vec<Vec<i32>> = Vec::new();
        if let Some(node) = root {
            let mut queue: Vec<Rc<RefCell<TreeNode>>> = Vec::new();
            queue.push(node);
            let mut is_reverse = false;
            
            while !queue.is_empty() {
                let mut level: Vec<i32> = Vec::new();
                let size = queue.len();
                
                for _ in 0..size {
                    let node = queue.remove(0);
                    let node = node.borrow();
                    level.push(node.val);
                    
                    if let Some(left) = &node.left {
                        queue.push(Rc::clone(left));
                    }
                    
                    if let Some(right) = &node.right {
                        queue.push(Rc::clone(right));
                    }
                }
                
                if is_reverse {
                    level.reverse();
                }
                
                result.push(level);
                is_reverse = !is_reverse;
            }
        }
        
        result

    }
}
