
use std::rc::Rc;
use std::cell::RefCell;

impl Solution {
    pub fn recover_tree(root: &mut Option<Rc<RefCell<TreeNode>>>) {
        let mut first: Option<Rc<RefCell<TreeNode>>> = None;
        let mut second: Option<Rc<RefCell<TreeNode>>> = None;
        let mut prev: Option<Rc<RefCell<TreeNode>>> = None;

        Self::inorder_traversal(root.as_ref(), &mut prev, &mut first, &mut second);

        if let (Some(first_node), Some(second_node)) = (first, second) {
            let temp = first_node.borrow().val;
            first_node.borrow_mut().val = second_node.borrow().val;
            second_node.borrow_mut().val = temp;
        }
    }

    fn inorder_traversal(
        root: Option<&Rc<RefCell<TreeNode>>>,
        prev: &mut Option<Rc<RefCell<TreeNode>>>,
        first: &mut Option<Rc<RefCell<TreeNode>>>,
        second: &mut Option<Rc<RefCell<TreeNode>>>,
    ) {
        if let Some(node) = root {
            Self::inorder_traversal(node.borrow().left.as_ref(), prev, first, second);

            if let Some(prev_node) = prev {
                if prev_node.borrow().val > node.borrow().val {
                    if first.is_none() {
                        *first = Some(prev_node.clone());
                    }
                    *second = Some(node.clone());
                }
            }
            *prev = Some(node.clone());

            Self::inorder_traversal(node.borrow().right.as_ref(), prev, first, second);
        }
    }
}
