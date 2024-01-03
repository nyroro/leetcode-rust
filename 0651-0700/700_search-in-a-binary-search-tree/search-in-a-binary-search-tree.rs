
use std::rc::Rc;
use std::cell::RefCell;

impl Solution {
    pub fn search_bst(root: Option<Rc<RefCell<TreeNode>>>, val: i32) -> Option<Rc<RefCell<TreeNode>>> {
        // 如果根节点为空，直接返回 null

        if root.is_none() {
            return None;
        }
        
        // 获取根节点的可变引用

        let root_ref = root.as_ref().unwrap().borrow();
        
        // 如果当前节点的值等于目标值，返回当前节点

        if root_ref.val == val {
            return root.clone();
        }
        
        // 如果目标值小于当前节点的值，递归地在左子树中查找

        if val < root_ref.val {
            return Solution::search_bst(root_ref.left.clone(), val);
        }
        // 如果目标值大于当前节点的值，递归地在右子树中查找

        else {
            return Solution::search_bst(root_ref.right.clone(), val);
        }
    }
}
