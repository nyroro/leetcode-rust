
use std::rc::Rc;
use std::cell::RefCell;

impl Solution {
    pub fn balance_bst(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        let mut values = Vec::new();
        Solution::inorder_traversal(root, &mut values);
        Solution::build_balanced_bst(&values)
    }
    
    fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>, values: &mut Vec<i32>) {
        if let Some(node) = root {
            let node_ref = node.borrow();
            Solution::inorder_traversal(node_ref.left.clone(), values);
            values.push(node_ref.val);
            Solution::inorder_traversal(node_ref.right.clone(), values);
        }
    }
    
    fn build_balanced_bst(values: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
        if values.is_empty() {
            return None;
        }
        let mid = values.len() / 2;
        let mut node = TreeNode::new(values[mid]);
        node.left = Solution::build_balanced_bst(&values[..mid]);
        node.right = Solution::build_balanced_bst(&values[mid + 1..]);
        Some(Rc::new(RefCell::new(node)))
    }
}
