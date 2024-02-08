
use std::rc::Rc;
use std::cell::RefCell;

impl Solution {
    pub fn is_sub_path(head: Option<Box<crate::ListNode>>, root: Option<Rc<RefCell<crate::TreeNode>>>) -> bool {
        // Helper function to check for path

        fn check_for_path(root: Option<&Rc<RefCell<crate::TreeNode>>>, head: Option<&Box<crate::ListNode>>) -> bool {
            match (root, head) {
                (Some(r), Some(h)) => {
                    let r_borrow = r.borrow();
                    if r_borrow.val == h.val {
                        check_for_path(r_borrow.left.as_ref(), h.next.as_ref()) || 
                        check_for_path(r_borrow.right.as_ref(), h.next.as_ref())
                    } else {
                        false

                    }
                }
                (None, Some(_)) => false,
                (_, None) => true,
            }
        }

        // Main function logic

        match (head, root) {
            (Some(h), Some(r)) => {
                let r_borrow = r.borrow();
                check_for_path(Some(&r), Some(&h)) || 
                Solution::is_sub_path(Some(h.clone()), r_borrow.left.clone()) || 
                Solution::is_sub_path(Some(h.clone()), r_borrow.right.clone())
            }
            (None, _) => true,
            (_, None) => false,
        }
    }
}
