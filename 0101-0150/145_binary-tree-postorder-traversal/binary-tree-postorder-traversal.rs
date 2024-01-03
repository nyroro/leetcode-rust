
use std::rc::Rc;
use std::cell::RefCell;

impl Solution {
    pub fn postorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut result = Vec::new();
        Solution::postorder_helper(&root, &mut result);
        result

    }

    fn postorder_helper(node: &Option<Rc<RefCell<TreeNode>>>, result: &mut Vec<i32>) {
        if let Some(n) = node {
            let n_ref = n.borrow();
            Solution::postorder_helper(&n_ref.left, result);
            Solution::postorder_helper(&n_ref.right, result);
            result.push(n_ref.val);
        }
    }
}
