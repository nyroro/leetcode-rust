
use std::rc::Rc;
use std::cell::RefCell;

impl Solution {
    pub fn sum_of_left_leaves(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        // 定义递归函数，参数为当前节点和是否为左子节点的标志

        fn dfs(node: Option<Rc<RefCell<TreeNode>>>, is_left: bool) -> i32 {
            match node {
                Some(inner) => {
                    let node = inner.borrow();
                    if node.left.is_none() && node.right.is_none() && is_left {
                        // 当前节点是左叶子节点，将其值返回

                        return node.val;
                    }
                    // 递归处理左子树和右子树，并将结果相加

                    dfs(node.left.clone(), true) + dfs(node.right.clone(), false)
                }
                None => 0, // 空节点返回0

            }
        }
        
        dfs(root, false) // 调用递归函数，初始时不是左子节点

    }
}
