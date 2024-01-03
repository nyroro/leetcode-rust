
// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
// pub struct TreeNode {
//   pub val: i32,
//   pub left: Option<Rc<RefCell<TreeNode>>>,
//   pub right: Option<Rc<RefCell<TreeNode>>>,
// }
// 
// impl TreeNode {
//   #[inline]
//   pub fn new(val: i32) -> Self {
//     TreeNode {
//       val,
//       left: None,
//       right: None

//     }
//   }
// }
use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn flip_equiv(root1: Option<Rc<RefCell<TreeNode>>>, root2: Option<Rc<RefCell<TreeNode>>>) -> bool {
        // 处理边界情况

        if root1.is_none() && root2.is_none() {
            return true;
        }
        if root1.is_none() || root2.is_none() || root1.as_ref().unwrap().borrow().val != root2.as_ref().unwrap().borrow().val {
            return false;
        }
        
        let left1 = root1.as_ref().unwrap().borrow().left.clone();
        let right1 = root1.as_ref().unwrap().borrow().right.clone();
        let left2 = root2.as_ref().unwrap().borrow().left.clone();
        let right2 = root2.as_ref().unwrap().borrow().right.clone();
        
        // 判断两个二叉树的左右子树是否是翻转等价的

        (Self::flip_equiv(left1.clone(), left2.clone()) && Self::flip_equiv(right1.clone(), right2.clone())) ||
        (Self::flip_equiv(left1.clone(), right2.clone()) && Self::flip_equiv(right1.clone(), left2.clone()))
    }
}
