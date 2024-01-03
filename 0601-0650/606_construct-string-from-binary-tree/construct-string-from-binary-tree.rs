
use std::rc::Rc;
use std::cell::RefCell;

impl Solution {
    pub fn tree2str(root: Option<Rc<RefCell<TreeNode>>>) -> String {
        match root {
            Some(node) => {
                let node_ref = node.borrow();
                let left = Solution::tree2str(node_ref.left.clone());
                let right = Solution::tree2str(node_ref.right.clone());
                if left == "" && right == "" {
                    node_ref.val.to_string()
                } else if right == "" {
                    format!("{}({})", node_ref.val, left)
                } else {
                    format!("{}({})({})", node_ref.val, left, right)
                }
            }
            None => "".to_string()
        }
    }
}
