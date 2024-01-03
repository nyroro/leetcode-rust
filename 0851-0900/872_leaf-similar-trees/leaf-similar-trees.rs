
use std::rc::Rc;
use std::cell::RefCell;

impl Solution {
    pub fn leaf_similar(root1: Option<Rc<RefCell<TreeNode>>>, root2: Option<Rc<RefCell<TreeNode>>>) -> bool {
        let mut leaves1 = Vec::new();
        let mut leaves2 = Vec::new();
        
        Self::dfs(&root1, &mut leaves1);
        Self::dfs(&root2, &mut leaves2);
        
        leaves1 == leaves2

    }
    
    fn dfs(root: &Option<Rc<RefCell<TreeNode>>>, leaves: &mut Vec<i32>) {
        if let Some(node) = root {
            let node_ref = node.borrow();
            if node_ref.left.is_none() && node_ref.right.is_none() {
                leaves.push(node_ref.val);
            }
            Self::dfs(&node_ref.left, leaves);
            Self::dfs(&node_ref.right, leaves);
        }
    }
}
