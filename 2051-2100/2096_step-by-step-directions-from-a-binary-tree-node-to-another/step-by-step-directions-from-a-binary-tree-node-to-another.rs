
use std::rc::Rc;
use std::cell::RefCell;

impl Solution {
    pub fn get_directions(root: Option<Rc<RefCell<TreeNode>>>, start_value: i32, dest_value: i32) -> String {
        fn find_path(node: &Option<Rc<RefCell<TreeNode>>>, target: i32, path: &mut Vec<char>) -> bool {
            if let Some(n) = node {
                let n_ref = n.borrow();
                if n_ref.val == target {
                    return true;
                }
                if find_path(&n_ref.left, target, path) {
                    path.push('L');
                    return true;
                }
                if find_path(&n_ref.right, target, path) {
                    path.push('R');
                    return true;
                }
            }
            false

        }

        let mut start_path = Vec::new();
        let mut dest_path = Vec::new();
        
        find_path(&root, start_value, &mut start_path);
        find_path(&root, dest_value, &mut dest_path);
        
        while let (Some(s), Some(d)) = (start_path.last(), dest_path.last()) {
            if s == d {
                start_path.pop();
                dest_path.pop();
            } else {
                break;
            }
        }
        
        let mut result = String::new();
        result.push_str(&"U".repeat(start_path.len()));
        result.push_str(&dest_path.iter().rev().collect::<String>());
        
        result

    }
}
