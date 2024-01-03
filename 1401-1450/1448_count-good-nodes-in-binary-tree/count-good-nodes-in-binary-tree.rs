
use std::rc::Rc;
use std::cell::RefCell;

impl Solution {
    pub fn good_nodes(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn dfs(node: &Option<Rc<RefCell<TreeNode>>>, max_val: i32) -> i32 {
            match node {
                Some(inner) => {
                    let inner = inner.borrow();
                    let mut count = 0;
                    if inner.val >= max_val {
                        count += 1;
                    }
                    count += dfs(&inner.left, max_val.max(inner.val));
                    count += dfs(&inner.right, max_val.max(inner.val));
                    count

                }
                None => 0

            }
        }
        
        dfs(&root, std::i32::MIN)
    }
}
