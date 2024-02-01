
use std::rc::Rc;
use std::cell::RefCell;

impl Solution {
    pub fn insert_into_bst(root: Option<Rc<RefCell<TreeNode>>>, val: i32) -> Option<Rc<RefCell<TreeNode>>> {
        if let Some(node) = root {
            let mut node_borrow = node.borrow_mut();
            if val < node_borrow.val {
                node_borrow.left = Solution::insert_into_bst(node_borrow.left.clone(), val);
            } else {
                node_borrow.right = Solution::insert_into_bst(node_borrow.right.clone(), val);
            }
            Some(Rc::clone(&node))
        } else {
            Some(Rc::new(RefCell::new(TreeNode::new(val))))
        }
    }
}
