
use std::rc::Rc;
use std::cell::RefCell;

impl Solution {
    pub fn subtree_with_all_deepest(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        fn find_deepest_nodes(root: &Option<Rc<RefCell<TreeNode>>>, depth: i32) -> (i32, Option<Rc<RefCell<TreeNode>>>) {
            match root {
                Some(node) => {
                    let left = find_deepest_nodes(&node.borrow().left, depth + 1);
                    let right = find_deepest_nodes(&node.borrow().right, depth + 1);
                    if left.0 == right.0 {
                        return (left.0, root.clone());
                    } else if left.0 > right.0 {
                        return left;
                    } else {
                        return right;
                    }
                }
                None => (depth, None),
            }
        }

        fn subtree_with_deepest(root: &Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
            match root {
                Some(node) => {
                    let (left_depth, left_node) = find_deepest_nodes(&node.borrow().left, 1);
                    let (right_depth, right_node) = find_deepest_nodes(&node.borrow().right, 1);
                    if left_depth == right_depth {
                        return root.clone();
                    } else if left_depth > right_depth {
                        return subtree_with_deepest(&node.borrow().left);
                    } else {
                        return subtree_with_deepest(&node.borrow().right);
                    }
                }
                None => None,
            }
        }

        subtree_with_deepest(&root)
    }
}
