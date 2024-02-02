
use std::rc::Rc;
use std::cell::RefCell;

impl Solution {
    pub fn smallest_from_leaf(root: Option<Rc<RefCell<TreeNode>>>) -> String {
        let mut result = String::from("z".repeat(8501)); // Initialize result with a large lexicographically large string

        let mut path = String::new();
        Self::find_smallest_path(root, &mut path, &mut result);
        result

    }

    fn find_smallest_path(node: Option<Rc<RefCell<TreeNode>>>, path: &mut String, result: &mut String) {
        if let Some(n) = node {
            let n = n.borrow();
            path.insert(0, (n.val as u8 + b'a') as char);
            if n.left.is_none() && n.right.is_none() {
                // Leaf node, compare the path with the result

                if *path < *result {
                    *result = path.clone();
                }
            } else {
                // Recursively traverse the left and right subtrees

                Self::find_smallest_path(n.left.clone(), path, result);
                Self::find_smallest_path(n.right.clone(), path, result);
            }
            path.remove(0);
        }
    }
}
