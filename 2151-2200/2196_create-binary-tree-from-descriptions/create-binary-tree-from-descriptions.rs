
use std::collections::{HashMap, HashSet};
use std::rc::Rc;
use std::cell::RefCell;
use std::collections::VecDeque;

impl Solution {
    pub fn create_binary_tree(descriptions: Vec<Vec<i32>>) -> Option<Rc<RefCell<TreeNode>>> {
        let mut parent_candidates = HashSet::new();
        let mut relationships = HashMap::new();

        // Finding all eligible candidates for root

        for description in &descriptions {
            parent_candidates.insert(description[0]);
        }

        // Eliminating all un-eligible candidates

        for description in &descriptions {
            parent_candidates.remove(&description[1]);
        }

        // Creating map using values to find children in constant time (since all values are unique)
        for (i, description) in descriptions.iter().enumerate() {
            let parent = description[0];
            let child = description[1];
            let is_left = description[2];

            relationships.entry(parent).or_insert((None, None));

            if is_left == 1 {
                relationships.get_mut(&parent).unwrap().0 = Some(child);
            } else {
                relationships.get_mut(&parent).unwrap().1 = Some(child);
            }
        }

        let root_val = *parent_candidates.iter().next().unwrap();
        let root = Rc::new(RefCell::new(TreeNode::new(root_val)));
        let mut queue = VecDeque::new();
        queue.push_back(root.clone());

        while !queue.is_empty() {
            let node = queue.pop_front().unwrap();
            let val = node.borrow().val;

            if let Some((left, right)) = relationships.get(&val) {
                if let Some(left_val) = left {
                    let left_node = Rc::new(RefCell::new(TreeNode::new(*left_val)));
                    node.borrow_mut().left = Some(left_node.clone());
                    queue.push_back(left_node);
                }
                if let Some(right_val) = right {
                    let right_node = Rc::new(RefCell::new(TreeNode::new(*right_val)));
                    node.borrow_mut().right = Some(right_node.clone());
                    queue.push_back(right_node);
                }
            }
        }

        Some(root)
    }
}
