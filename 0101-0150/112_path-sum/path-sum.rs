
use std::rc::Rc;
use std::cell::RefCell;

impl Solution {
    pub fn has_path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> bool {
        if let Some(node) = root {
            let mut stack = vec![(node, 0)];
            while let Some((cur_node, cur_sum)) = stack.pop() {
                let node = cur_node.borrow();
                let sum = cur_sum + node.val;
                if node.left.is_none() && node.right.is_none() {
                    if sum == target_sum {
                        return true;
                    }
                }
                if let Some(left) = &node.left {
                    stack.push((left.clone(), sum));
                }
                if let Some(right) = &node.right {
                    stack.push((right.clone(), sum));
                }
            }
        }
        false

    }
}
