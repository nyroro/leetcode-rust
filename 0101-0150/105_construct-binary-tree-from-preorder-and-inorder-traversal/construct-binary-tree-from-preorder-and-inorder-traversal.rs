
use std::rc::Rc;
use std::cell::RefCell;

impl Solution {
    pub fn build_tree(preorder: Vec<i32>, inorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        if preorder.is_empty() || inorder.is_empty() {
            return None;
        }
        
        // 找到根节点的值

        let root_val = preorder[0];
        
        // 在中序遍历数组中找到根节点的位置

        let root_index = inorder.iter().position(|&x| x == root_val).unwrap();
        
        // 构建根节点

        let root = Some(Rc::new(RefCell::new(TreeNode::new(root_val))));
        
        // 递归构建左子树和右子树

        root.as_ref().unwrap().borrow_mut().left = Solution::build_tree(preorder[1..=root_index].to_vec(), inorder[..root_index].to_vec());
        root.as_ref().unwrap().borrow_mut().right = Solution::build_tree(preorder[root_index+1..].to_vec(), inorder[root_index+1..].to_vec());
        
        root

    }
}
