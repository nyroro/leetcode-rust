
use std::rc::Rc;
use std::cell::RefCell;

impl Solution {
    pub fn add_one_row(root: Option<Rc<RefCell<TreeNode>>>, val: i32, depth: i32) -> Option<Rc<RefCell<TreeNode>>> {
        if depth == 1 {
            let new_root = Some(Rc::new(RefCell::new(TreeNode::new(val))));
            new_root.as_ref().unwrap().borrow_mut().left = root;
            return new_root;
        }
        
        Self::add_row_helper(root, val, depth - 1)
    }
    
    fn add_row_helper(root: Option<Rc<RefCell<TreeNode>>>, val: i32, depth: i32) -> Option<Rc<RefCell<TreeNode>>> {
        if let Some(node) = root {
            if depth == 1 {
                let left_child = Some(Rc::new(RefCell::new(TreeNode::new(val))));
                let right_child = Some(Rc::new(RefCell::new(TreeNode::new(val))));
                left_child.as_ref().unwrap().borrow_mut().left = node.borrow_mut().left.take();
                right_child.as_ref().unwrap().borrow_mut().right = node.borrow_mut().right.take();
                node.borrow_mut().left = left_child;
                node.borrow_mut().right = right_child;
            } else {
                let left = Self::add_row_helper(node.borrow_mut().left.take(), val, depth - 1);
                let right = Self::add_row_helper(node.borrow_mut().right.take(), val, depth - 1);
                node.borrow_mut().left = left;
                node.borrow_mut().right = right;
            }
            Some(node)
        } else {
            None

        }
    }
}
