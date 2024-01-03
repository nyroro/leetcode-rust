
use std::rc::Rc;
use std::cell::RefCell;

impl Solution {
    pub fn construct_from_pre_post(preorder: Vec<i32>, postorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        fn build_tree(preorder: &[i32], postorder: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
            if preorder.is_empty() {
                return None;
            }
            
            let root_val = preorder[0];
            let root = Some(Rc::new(RefCell::new(TreeNode::new(root_val))));
            
            if preorder.len() == 1 {
                return root;
            }
            
            let left_val = preorder[1];
            let left_index = postorder.iter().position(|&x| x == left_val).unwrap();
            
            let left_preorder = &preorder[1..=left_index+1];
            let left_postorder = &postorder[..=left_index];
            
            let right_preorder = &preorder[left_index+2..];
            let right_postorder = &postorder[left_index+1..postorder.len()-1];
            
            root.as_ref().unwrap().borrow_mut().left = build_tree(left_preorder, left_postorder);
            root.as_ref().unwrap().borrow_mut().right = build_tree(right_preorder, right_postorder);
            
            root

        }
        
        build_tree(&preorder, &postorder)
    }
}
