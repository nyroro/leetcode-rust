
use std::rc::Rc;
use std::cell::RefCell;

// Definition for a binary tree node.
use crate::tree_node::TreeNode;  // Assuming the TreeNode is defined in a module called tree_node




impl Solution {
    pub fn min_camera_cover(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn solve(root: &Option<Rc<RefCell<TreeNode>>>, ans: &mut i32) -> i32 {
            if let Some(node) = root {
                let node_ref = node.borrow();
                let l = solve(&node_ref.left, ans);
                let r = solve(&node_ref.right, ans);

                if l == -1 {
                    if r == -1 {
                        return 0;
                    } else if r == 0 {
                        *ans += 1;
                        return 1;
                    } else {
                        return -1;
                    }
                } else if l == 0 {
                    *ans += 1;
                    return 1;
                } else {
                    if r == -1 {
                        return -1;
                    } else if r == 0 {
                        *ans += 1;
                        return 1;
                    } else {
                        return -1;
                    }
                }
            } else {
                return -1;
            }
        }

        let mut ans = 0;
        let x = solve(&root, &mut ans);
        if x == 0 {
            ans += 1;
        }
        ans

    }
}
