
use std::rc::Rc;
use std::cell::RefCell;

impl Solution {
    pub fn merge_trees(root1: Option<Rc<RefCell<TreeNode>>>, root2: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        match (root1, root2) {
            (Some(node1), Some(node2)) => {
                let val = node1.borrow().val + node2.borrow().val;
                let left = Solution::merge_trees(node1.borrow().left.clone(), node2.borrow().left.clone());
                let right = Solution::merge_trees(node1.borrow().right.clone(), node2.borrow().right.clone());
                Some(Rc::new(RefCell::new(TreeNode {
                    val,
                    left,
                    right,
                })))
            }
            (Some(node), None) => Some(node.clone()),
            (None, Some(node)) => Some(node.clone()),
            (None, None) => None,
        }
    }
}
