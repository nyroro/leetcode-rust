
use std::cell::RefCell;
use std::rc::Rc;

struct BSTIterator {
    stack: Vec<Rc<RefCell<TreeNode>>>,
}

impl BSTIterator {
    fn new(root: Option<Rc<RefCell<TreeNode>>>) -> Self {
        let mut stack = Vec::new();
        let mut node = root;
        while let Some(n) = node {
            stack.push(n.clone());
            node = n.borrow().left.clone();
        }
        BSTIterator { stack }
    }
    
    fn next(&mut self) -> i32 {
        let node = self.stack.pop().unwrap();
        let val = node.borrow().val;
        let mut right = node.borrow_mut().right.take();
        while let Some(n) = right {
            self.stack.push(n.clone());
            right = n.borrow_mut().left.take();
        }
        val

    }
    
    fn has_next(&self) -> bool {
        !self.stack.is_empty()
    }
}
