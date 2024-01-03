
use std::rc::Rc;
use std::cell::RefCell;

impl Solution {
    pub fn min_diff_in_bst(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut values = Vec::new();
        Self::inorder_traversal(root.as_ref(), &mut values);
        let mut min_diff = i32::max_value();
        for i in 1..values.len() {
            min_diff = min_diff.min(values[i] - values[i - 1]);
        }
        min_diff

    }
    
    fn inorder_traversal(root: Option<&Rc<RefCell<TreeNode>>>, values: &mut Vec<i32>) {
        if let Some(node) = root {
            let node_ref = node.borrow();
            Self::inorder_traversal(node_ref.left.as_ref(), values);
            values.push(node_ref.val);
            Self::inorder_traversal(node_ref.right.as_ref(), values);
        }
    }
}
