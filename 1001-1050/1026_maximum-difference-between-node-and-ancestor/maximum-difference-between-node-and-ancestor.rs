
use std::rc::Rc;
use std::cell::RefCell;

// pub struct TreeNode {
//   pub val: i32,
//   pub left: Option<Rc<RefCell<TreeNode>>>,
//   pub right: Option<Rc<RefCell<TreeNode>>>,
// }

impl Solution {
    pub fn max_ancestor_diff(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn find_max_diff(node: Option<&Rc<RefCell<TreeNode>>>, min: i32, max: i32) -> i32 {
            match node {
                Some(n) => {
                    let val = n.borrow().val;
                    let diff = (val - min).abs().max((val - max).abs());
                    let min = min.min(val);
                    let max = max.max(val);
                    diff.max(find_max_diff(n.borrow().left.as_ref(), min, max))
                        .max(find_max_diff(n.borrow().right.as_ref(), min, max))
                }
                None => 0

            }
        }
        
        find_max_diff(root.as_ref(), root.as_ref().unwrap().borrow().val, root.as_ref().unwrap().borrow().val)
    }
}
