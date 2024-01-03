
use std::rc::Rc;
use std::cell::RefCell;

// Definition for a binary tree node.
// pub struct TreeNode {
//   pub val: i32,
//   pub left: Option<Rc<RefCell<TreeNode>>>,
//   pub right: Option<Rc<RefCell<TreeNode>>>,
// }

impl Solution {
    pub fn count_nodes(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if root.is_none() {
            return 0;
        }
        
        let mut height = 0;
        let mut node = root.clone();
        
        while node.as_ref().unwrap().borrow().left.is_some() {
            height += 1;
            node = node.unwrap().borrow().left.clone();
        }
        
        let mut left = 1 << height;
        let mut right = (1 << (height + 1)) - 1;
        
        while left < right {
            let mid = (right - left + 1) / 2 + left;
            if Self::exists(root.clone(), mid, height) {
                left = mid;
            } else {
                right = mid - 1;
            }
        }
        
        left as i32

    }
    
    fn exists(root: Option<Rc<RefCell<TreeNode>>>, mut idx: i32, mut height: i32) -> bool {
        let mut node = root;
        let mut mask = 1 << (height - 1);
        
        while node.is_some() && mask > 0 {
            if idx & mask == 0 {
                node = node.unwrap().borrow().left.clone();
            } else {
                node = node.unwrap().borrow().right.clone();
            }
            mask >>= 1;
        }
        
        node.is_some()
    }
}
