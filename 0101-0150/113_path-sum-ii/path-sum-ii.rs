
use std::rc::Rc;
use std::cell::RefCell;

impl Solution {
    pub fn path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> Vec<Vec<i32>> {
        let mut result = Vec::new();
        let mut path = Vec::new();
        Self::dfs(&root, target_sum, 0, &mut path, &mut result);
        result

    }
    
    fn dfs(node: &Option<Rc<RefCell<TreeNode>>>, target_sum: i32, current_sum: i32, path: &mut Vec<i32>, result: &mut Vec<Vec<i32>>) {
        if let Some(node) = node {
            let node = node.borrow();
            let current_sum = current_sum + node.val;
            path.push(node.val);
            
            if node.left.is_none() && node.right.is_none() && current_sum == target_sum {
                result.push(path.clone());
            }
            
            Self::dfs(&node.left, target_sum, current_sum, path, result);
            Self::dfs(&node.right, target_sum, current_sum, path, result);
            
            path.pop();
        }
    }
}
