
use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;

impl Solution {
    pub fn find_mode(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut count_map: HashMap<i32, i32> = HashMap::new();
        let mut max_count = 0;

        // 中序遍历

        Self::inorder_traversal(&root, &mut count_map, &mut max_count);

        // 找到出现次数最多的值

        let mut result: Vec<i32> = Vec::new();
        for (val, count) in &count_map {
            if *count == max_count {
                result.push(*val);
            }
        }

        result

    }

    fn inorder_traversal(node: &Option<Rc<RefCell<TreeNode>>>, count_map: &mut HashMap<i32, i32>, max_count: &mut i32) {
        if let Some(n) = node {
            let n = n.borrow();
            Self::inorder_traversal(&n.left, count_map, max_count);
            let count = count_map.entry(n.val).or_insert(0);
            *count += 1;
            *max_count = (*max_count).max(*count);
            Self::inorder_traversal(&n.right, count_map, max_count);
        }
    }
}
