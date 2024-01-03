
use std::rc::Rc;
use std::cell::RefCell;

impl Solution {
    pub fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        // 定义一个辅助函数，用于递归判断节点是否满足BST的条件

        fn is_valid(node: Option<&Rc<RefCell<TreeNode>>>, min: Option<i32>, max: Option<i32>) -> bool {
            if let Some(node) = node {
                let node = node.borrow();
                // 判断节点值是否在[min, max]的范围内

                if let Some(min_val) = min {
                    if node.val <= min_val {
                        return false;
                    }
                }
                if let Some(max_val) = max {
                    if node.val >= max_val {
                        return false;
                    }
                }
                // 递归判断左右子树

                return is_valid(node.left.as_ref(), min, Some(node.val)) && is_valid(node.right.as_ref(), Some(node.val), max);
            }
            true

        }
        
        is_valid(root.as_ref(), None, None)
    }
}
