
use std::rc::Rc;
use std::cell::RefCell;

impl Solution {
    pub fn insert_into_max_tree(root: Option<Rc<RefCell<TreeNode>>>, val: i32) -> Option<Rc<RefCell<TreeNode>>> {
        match root {
            Some(node) => {
                if val > node.borrow().val {
                    let new_node = Some(Rc::new(RefCell::new(TreeNode::new(val))));
                    new_node.as_ref().unwrap().borrow_mut().left = Some(node);
                    new_node

                } else {
                    let right = node.borrow_mut().right.take();
                    node.borrow_mut().right = Self::insert_into_max_tree(right, val);
                    Some(node)
                }
            },
            None => Some(Rc::new(RefCell::new(TreeNode::new(val))))
        }
    }
}
