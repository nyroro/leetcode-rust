
use std::rc::Rc;
use std::cell::RefCell;

// Definition for a binary tree node.
// pub struct TreeNode {
//   pub val: i32,
//   pub left: Option<Rc<RefCell<TreeNode>>>,
//   pub right: Option<Rc<RefCell<TreeNode>>>,
// }

impl Solution {
    pub fn average_of_subtree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn dfs(node: &Option<Rc<RefCell<TreeNode>>>, count: &mut i32, sum: &mut i32) {
            if let Some(n) = node {
                let n = n.borrow();
                *count += 1;
                *sum += n.val;
                dfs(&n.left, count, sum);
                dfs(&n.right, count, sum);
            }
        }
        
        fn average(node: &Option<Rc<RefCell<TreeNode>>>) -> i32 {
            let mut count = 0;
            let mut sum = 0;
            dfs(node, &mut count, &mut sum);
            if count == 0 {
                return 0;
            }
            sum / count

        }
        
        fn traverse(node: &Option<Rc<RefCell<TreeNode>>>, total: &mut i32) {
            if let Some(n) = node {
                let n = n.borrow();
                if n.val == average(node) {
                    *total += 1;
                }
                traverse(&n.left, total);
                traverse(&n.right, total);
            }
        }
        
        let mut total = 0;
        traverse(&root, &mut total);
        total

    }
}
