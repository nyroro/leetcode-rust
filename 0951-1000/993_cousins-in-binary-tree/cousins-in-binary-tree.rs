
use std::rc::Rc;
use std::cell::RefCell;

// pub struct TreeNode {
//   pub val: i32,
//   pub left: Option<Rc<RefCell<TreeNode>>>,
//   pub right: Option<Rc<RefCell<TreeNode>>>,
// }

impl Solution {
    pub fn is_cousins(root: Option<Rc<RefCell<TreeNode>>>, x: i32, y: i32) -> bool {
        fn dfs(node: &Option<Rc<RefCell<TreeNode>>>, parent: i32, depth: i32, value: i32) -> (i32, i32) {
            if let Some(n) = node {
                let n = n.borrow();
                if n.val == value {
                    return (parent, depth);
                }
                let (left_parent, left_depth) = dfs(&n.left, n.val, depth + 1, value);
                if left_depth != 0 {
                    return (left_parent, left_depth);
                }
                dfs(&n.right, n.val, depth + 1, value)
            } else {
                (0, 0)
            }
        }
        
        let (x_parent, x_depth) = dfs(&root, 0, 0, x);
        let (y_parent, y_depth) = dfs(&root, 0, 0, y);
        
        x_parent != y_parent && x_depth == y_depth

    }
}
