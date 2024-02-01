
use std::rc::Rc;
use std::cell::RefCell;

impl Solution {
    pub fn all_possible_fbt(n: i32) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
        if n % 2 == 0 {
            return vec![];
        }
        if n == 1 {
            return vec![Some(Rc::new(RefCell::new(TreeNode::new(0)))];
        }
        let mut result = vec![];
        for i in (1..n).step_by(2) {
            let left_trees = Self::all_possible_fbt(i);
            let right_trees = Self::all_possible_fbt(n - 1 - i);
            for left in &left_trees {
                for right in &right_trees {
                    let root = Some(Rc::new(RefCell::new(TreeNode::new(0)));
                    root.borrow_mut().left = left.clone();
                    root.borrow_mut().right = right.clone();
                    result.push(root);
                }
            }
        }
        result

    }
}
