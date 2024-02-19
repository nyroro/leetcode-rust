
use std::collections::{HashMap, VecDeque};
use std::rc::Rc;
use std::cell::RefCell;

impl Solution {
    pub fn vertical_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let mut result: Vec<Vec<i32>> = Vec::new();
        let mut queue: VecDeque<(Rc<RefCell<TreeNode>>, i32, i32)> = VecDeque::new();
        let mut map: HashMap<i32, HashMap<i32, Vec<i32>>> = HashMap::new();

        // Perform level order traversal

        if let Some(root) = root {
            queue.push_back((root.clone(), 0, 0));
            while !queue.is_empty() {
                if let Some((node, row, col)) = queue.pop_front() {
                    map.entry(col).or_insert(HashMap::new()).entry(row).or_insert(Vec::new()).push(node.borrow().val);

                    if let Some(left) = node.borrow().left.clone() {
                        queue.push_back((left, row + 1, col - 1));
                    }
                    if let Some(right) = node.borrow().right.clone() {
                        queue.push_back((right, row + 1, col + 1));
                    }
                }
            }
        }

        // Sort the nodes within each column and construct the result

        let mut columns: Vec<i32> = map.keys().cloned().collect();
        columns.sort();
        for col in columns {
            let mut rows: Vec<i32> = map[&col].keys().cloned().collect();
            rows.sort();
            let mut column_nodes: Vec<i32> = Vec::new();
            for row in rows {
                let mut nodes: Vec<i32> = map[&col][&row].clone();
                nodes.sort();
                column_nodes.extend(nodes);
            }
            result.push(column_nodes);
        }

        result

    }
}
