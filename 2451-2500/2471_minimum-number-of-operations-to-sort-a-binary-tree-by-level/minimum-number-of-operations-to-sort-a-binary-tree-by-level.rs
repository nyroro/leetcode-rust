
use std::rc::Rc;
use std::cell::RefCell;
use std::collections::VecDeque;

impl Solution {
    pub fn minimum_operations(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut queue = VecDeque::new();
        queue.push_back(root);
        let mut ans = 0;
        
        while !queue.is_empty() {
            let n = queue.len();
            let mut row = Vec::new();
            
            for _ in 0..n {
                if let Some(node) = queue.pop_front() {
                    let node = node.borrow();
                    row.push(node.val);
                    
                    if let Some(left) = &node.left {
                        queue.push_back(Some(left.clone()));
                    }
                    if let Some(right) = &node.right {
                        queue.push_back(Some(right.clone()));
                    }
                }
            }
            
            ans += Self::calculate_min_operations(&mut row);
        }
        
        ans

    }
    
    fn calculate_min_operations(row: &mut Vec<i32>) -> i32 {
        let mut operations = 0;
        let mut sorted_row = row.clone();
        sorted_row.sort_unstable();
        
        for i in 0..row.len() {
            if row[i] != sorted_row[i] {
                let mut j = i + 1;
                while row[j] != sorted_row[i] {
                    j += 1;
                }
                row.swap(i, j);
                operations += 1;
            }
        }
        
        operations

    }
}
