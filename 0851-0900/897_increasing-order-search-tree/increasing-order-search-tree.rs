
use std::rc::Rc;
use std::cell::RefCell;

impl Solution {
    pub fn increasing_bst(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        let mut values = Vec::new();
        Solution::inorder_traversal(root.clone(), &mut values);
        Solution::build_bst(values)
    }
    
    fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>, values: &mut Vec<i32>) {
        if let Some(node) = root {
            let node_ref = node.borrow();
            Solution::inorder_traversal(node_ref.left.clone(), values);
            values.push(node_ref.val);
            Solution::inorder_traversal(node_ref.right.clone(), values);
        }
    }
    
    fn build_bst(values: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        let new_root = Rc::new(RefCell::new(TreeNode::new(0)));
        let mut current = new_root.clone();
        
        for val in values {
            let new_node = Rc::new(RefCell::new(TreeNode::new(val)));
            current.borrow_mut().right = Some(new_node.clone());
            current = new_node;
        }
        
        let result = new_root.borrow().right.clone();
        new_root.borrow_mut().right = None;
        result

    }
}
