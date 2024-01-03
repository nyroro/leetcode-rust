
use std::rc::Rc;
use std::cell::RefCell;

impl Solution {
    pub fn sum_numbers(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn dfs(node: &Option<Rc<RefCell<TreeNode>>>, current_sum: i32) -> i32 {
            if let Some(n) = node {
                let n = n.borrow();
                let current_sum = current_sum * 10 + n.val;
                if n.left.is_none() && n.right.is_none() {
                    return current_sum;
                }
                return dfs(&n.left, current_sum) + dfs(&n.right, current_sum);
            }
            0

        }
        
        dfs(&root, 0)
    }
}
