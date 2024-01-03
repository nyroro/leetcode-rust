
use std::rc::Rc;
use std::cell::RefCell;

impl Solution {
    pub fn kth_smallest(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> i32 {
        let mut result = 0;
        let mut count = 0;
        
        fn inorder_traversal(node: &Option<Rc<RefCell<TreeNode>>>, k: i32, count: &mut i32, result: &mut i32) {
            if let Some(n) = node {
                inorder_traversal(&n.borrow().left, k, count, result);
                *count += 1;
                if *count == k {
                    *result = n.borrow().val;
                }
                inorder_traversal(&n.borrow().right, k, count, result);
            }
        }
        
        inorder_traversal(&root, k, &mut count, &mut result);
        result

    }
}
