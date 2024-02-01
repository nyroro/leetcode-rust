
use std::rc::Rc;
use std::cell::RefCell;

impl Solution {
    pub fn btree_game_winning_move(root: Option<Rc<RefCell<TreeNode>>>, n: i32, x: i32) -> bool {
        let mut left = 0;
        let mut right = 0;
        
        // 定义递归函数来计算节点的子节点数量

        fn count(node: Option<Rc<RefCell<TreeNode>>>, val: i32) -> i32 {
            if let Some(n) = node {
                let n = n.borrow();
                let l = count(n.left.clone(), val);
                let r = count(n.right.clone(), val);
                if n.val == val {
                    left = l;
                    right = r;
                }
                return l + r + 1;
            }
            return 0;
        }
        
        // 调用递归函数计算节点的子节点数量

        let _ = count(root, x);
        
        // 根据题目要求的规则判断是否存在一种选择能够确保获胜

        return std::cmp::max(std::cmp::max(left, right), n - left - right - 1) > n / 2;
    }
}
