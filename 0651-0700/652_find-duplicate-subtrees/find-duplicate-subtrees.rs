
use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap; // 导入HashMap


impl Solution {
    pub fn find_duplicate_subtrees(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
        let mut map: HashMap<String, i32> = HashMap::new();
        let mut res: Vec<Option<Rc<RefCell<TreeNode>>>> = Vec::new();

        Self::traverse(&root, &mut map, &mut res);

        res

    }

    fn traverse(root: &Option<Rc<RefCell<TreeNode>>>, map: &mut HashMap<String, i32>, res: &mut Vec<Option<Rc<RefCell<TreeNode>>>>) -> String {
        if let Some(node) = root {
            let node = node.borrow();
            let left = Self::traverse(&node.left, map, res);
            let right = Self::traverse(&node.right, map, res);
            let subtree = format!("{} {} {}", node.val, left, right);

            let count = map.entry(subtree.clone()).or_insert(0);
            *count += 1;

            if *count == 2 {
                res.push(root.clone());
            }

            subtree

        } else {
            "#".to_string()
        }
    }
}
