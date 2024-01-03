
use std::rc::Rc;
use std::cell::RefCell;

impl Solution {
    pub fn is_subtree(root: Option<Rc<RefCell<TreeNode>>>, sub_root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        // Check if the current node is a match or if the subtree exists in the left or right child nodes

        fn is_match(s: Option<&Rc<RefCell<TreeNode>>>, t: Option<&Rc<RefCell<TreeNode>>>) -> bool {
            match (s, t) {
                (None, None) => true,
                (Some(s), Some(t)) => {
                    s.borrow().val == t.borrow().val

                        && is_match(s.borrow().left.as_ref(), t.borrow().left.as_ref())
                        && is_match(s.borrow().right.as_ref(), t.borrow().right.as_ref())
                }
                _ => false,
            }
        }

        // Traverse the tree and check for a match or the existence of the subtree

        fn traverse(s: Option<&Rc<RefCell<TreeNode>>>, t: Option<&Rc<RefCell<TreeNode>>>) -> bool {
            match s {
                None => false,
                Some(s) => {
                    is_match(Some(s), t) || traverse(s.borrow().left.as_ref(), t) || traverse(s.borrow().right.as_ref(), t)
                }
            }
        }

        traverse(root.as_ref(), sub_root.as_ref())
    }
}
