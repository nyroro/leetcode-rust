
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
use std::collections::HashMap;



impl Solution {
    pub fn kth_largest_level_sum(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> i64 {
        let mut level_sums: HashMap<i32, i64> = HashMap::new();
        let mut max_level = 0;
        
        // 辅助函数：遍历二叉树并计算每一层的节点值之和

        fn traverse_tree(node: Option<Rc<RefCell<TreeNode>>>, level: i32, level_sums: &mut HashMap<i32, i64>, max_level: &mut i32) {
            if let Some(n) = node {
                let val = n.borrow().val;
                *max_level = (*max_level).max(level);
                *level_sums.entry(level).or_insert(0) += val as i64;
                traverse_tree(n.borrow().left.clone(), level + 1, level_sums, max_level);
                traverse_tree(n.borrow().right.clone(), level + 1, level_sums, max_level);
            }
        }
        
        traverse_tree(root, 1, &mut level_sums, &mut max_level);
        
        // 将哈希表中的值排序以找到第 k 大的层级和

        let mut sorted_sums: Vec<i64> = level_sums.values().cloned().collect();
        sorted_sums.sort_unstable_by(|a, b| b.cmp(a));
        
        if let Some(&result) = sorted_sums.get((k - 1) as usize) {
            result

        } else {
            -1

        }
    }
}
