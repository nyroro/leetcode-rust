
use std::rc::Rc;
use std::cell::RefCell;

impl Solution {
    pub fn build_tree(inorder: Vec<i32>, postorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        if inorder.is_empty() || postorder.is_empty() {
            return None;
        }
        
        let root_val = postorder[postorder.len() - 1];
        let root = Some(Rc::new(RefCell::new(TreeNode::new(root_val))));
        
        let root_index = inorder.iter().position(|&x| x == root_val).unwrap();
        
        let left_inorder = &inorder[..root_index];
        let right_inorder = &inorder[root_index + 1..];
        
        let left_postorder = &postorder[..left_inorder.len()];
        let right_postorder = &postorder[left_inorder.len()..postorder.len() - 1];
        
        root.as_ref().unwrap().borrow_mut().left = Solution::build_tree(left_inorder.to_vec(), left_postorder.to_vec());
        root.as_ref().unwrap().borrow_mut().right = Solution::build_tree(right_inorder.to_vec(), right_postorder.to_vec());
        
        root

    }
}
