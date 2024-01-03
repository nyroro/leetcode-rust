
use std::rc::Rc;
use std::cell::RefCell;

impl Solution {
    pub fn find_second_minimum_value(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut values = Vec::new();
        Solution::dfs(root, &mut values);
        
        values.sort();
        
        let mut second_min = -1;
        let mut found_second_min = false;
        
        for i in 1..values.len() {
            if values[i] > values[0] {
                second_min = values[i];
                found_second_min = true;
                break;
            }
        }
        
        if found_second_min {
            second_min

        } else {
            -1

        }
    }
    
    fn dfs(root: Option<Rc<RefCell<TreeNode>>>, values: &mut Vec<i32>) {
        if let Some(node) = root {
            values.push(node.borrow().val);
            Solution::dfs(node.borrow().left.clone(), values);
            Solution::dfs(node.borrow().right.clone(), values);
        }
    }
}
