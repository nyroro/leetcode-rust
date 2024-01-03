
use std::rc::Rc;
use std::cell::RefCell;

impl Solution {
    pub fn get_all_elements(root1: Option<Rc<RefCell<TreeNode>>>, root2: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut result = Vec::new();
        
        // 中序遍历函数

        fn inorder_traversal(node: Option<Rc<RefCell<TreeNode>>>, result: &mut Vec<i32>) {
            if let Some(n) = node {
                let n = n.borrow();
                inorder_traversal(n.left.clone(), result);
                result.push(n.val);
                inorder_traversal(n.right.clone(), result);
            }
        }
        
        // 遍历第一个二叉搜索树

        inorder_traversal(root1, &mut result);
        
        // 遍历第二个二叉搜索树

        inorder_traversal(root2, &mut result);
        
        // 对结果数组进行排序

        result.sort();
        
        result

    }
}
