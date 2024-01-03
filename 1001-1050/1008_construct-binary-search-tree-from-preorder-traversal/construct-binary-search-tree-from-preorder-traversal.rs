
use std::rc::Rc;
use std::cell::RefCell;

impl Solution {
    pub fn bst_from_preorder(preorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        fn build_tree(preorder: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
            if preorder.is_empty() {
                return None;
            }
            
            let root_val = preorder[0];
            let mut root = TreeNode::new(root_val);
            
            let mut i = 1;
            while i < preorder.len() && preorder[i] < root_val {
                i += 1;
            }
            
            root.left = build_tree(&preorder[1..i]);
            root.right = build_tree(&preorder[i..]);
            
            Some(Rc::new(RefCell::new(root)))
        }
        
        build_tree(&preorder)
    }
}
