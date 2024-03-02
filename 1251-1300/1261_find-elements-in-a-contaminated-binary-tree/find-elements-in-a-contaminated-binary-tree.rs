
// In file find_elements.rs


use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashSet;

use crate::tree_node::TreeNode; // Assuming TreeNode is defined in a separate module


pub struct FindElements {
    num: HashSet<i32>,
}

impl FindElements {
    pub fn new(root: Option<Rc<RefCell<TreeNode>>>) -> Self {
        let mut find_elements = FindElements {
            num: HashSet::new(),
        };
        find_elements.traverse(&root, 0);
        find_elements

    }

    fn traverse(&mut self, node: &Option<Rc<RefCell<TreeNode>>>, val: i32) {
        if let Some(n) = node {
            let mut n = n.borrow_mut();
            n.val = val;
            self.num.insert(val);
            self.traverse(&n.left, 2 * val + 1);
            self.traverse(&n.right, 2 * val + 2);
        }
    }

    pub fn find(&self, target: i32) -> bool {
        self.num.contains(&target)
    }
}
