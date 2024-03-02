
use std::rc::Rc;
use std::cell::RefCell;

// ... (ListNode and TreeNode definitions remain the same)



impl Solution {
    pub fn sorted_list_to_bst(head: Option<Box<ListNode>>) -> Option<Rc<RefCell<TreeNode>>> {
        // Convert the linked list to a vector

        let mut values = Vec::new();
        let mut current = head;
        while let Some(node) = current {
            values.push(node.val);
            current = node.next;
        }
        
        // Recursive function to build the BST

        fn build_bst(values: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
            if values.is_empty() {
                return None;
            }
            let mid = values.len() / 2;
            let mut node = TreeNode::new(values[mid]);
            node.left = build_bst(&values[..mid]);
            node.right = build_bst(&values[mid + 1..]);
            Some(Rc::new(RefCell::new(node)))
        }
        
        build_bst(&values)
    }
}
