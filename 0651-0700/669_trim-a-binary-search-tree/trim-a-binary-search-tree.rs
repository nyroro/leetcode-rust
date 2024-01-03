
use std::rc::Rc;
use std::cell::RefCell;

impl Solution {
    pub fn trim_bst(root: Option<Rc<RefCell<TreeNode>>>, low: i32, high: i32) -> Option<Rc<RefCell<TreeNode>>> {
        match root {
            Some(node) => {
                let val = node.borrow().val;
                if val < low {
                    return Solution::trim_bst(node.borrow().right.clone(), low, high);
                } else if val > high {
                    return Solution::trim_bst(node.borrow().left.clone(), low, high);
                } else {
                    let left = Solution::trim_bst(node.borrow().left.clone(), low, high);
                    let right = Solution::trim_bst(node.borrow().right.clone(), low, high);
                    let mut new_node = TreeNode::new(val);
                    new_node.left = left;
                    new_node.right = right;
                    return Some(Rc::new(RefCell::new(new_node)));
                }
            }
            None => None,
        }
    }
}
