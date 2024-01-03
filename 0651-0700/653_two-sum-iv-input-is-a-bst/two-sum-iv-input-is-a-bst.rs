
use std::rc::Rc;
use std::cell::RefCell;

impl Solution {
    pub fn find_target(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> bool {
        fn find_two_sum(node: &Option<Rc<RefCell<TreeNode>>>, k: i32, set: &mut std::collections::HashSet<i32>) -> bool {
            if let Some(n) = node {
                let val = n.borrow().val;
                if set.contains(&(k - val)) {
                    return true;
                }
                set.insert(val);
                return find_two_sum(&n.borrow().left, k, set) || find_two_sum(&n.borrow().right, k, set);
            }
            false

        }
        
        let mut set = std::collections::HashSet::new();
        find_two_sum(&root, k, &mut set)
    }
}
