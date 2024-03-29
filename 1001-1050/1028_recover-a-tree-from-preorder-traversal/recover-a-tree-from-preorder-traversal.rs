
use std::rc::Rc;
use std::cell::RefCell;

impl Solution {
    pub fn recover_from_preorder(traversal: String) -> Option<Rc<RefCell<TreeNode>>> {
        let mut stack: Vec<(i32, Rc<RefCell<TreeNode>>)> = Vec::new();
        let mut i = 0;
        let n = traversal.len() as i32;
        
        while i < n {
            let mut level = 0;
            while i < n && &traversal[i as usize..=i as usize] == "-" {
                level += 1;
                i += 1;
            }
            
            let mut value = 0;
            while i < n && &traversal[i as usize..=i as usize] != "-" {
                value = value * 10 + (&traversal[i as usize..=i as usize]).parse::<i32>().unwrap();
                i += 1;
            }
            
            while stack.len() > level as usize {
                stack.pop();
            }
            
            let node = Rc::new(RefCell::new(TreeNode::new(value)));
            if let Some((_, parent)) = stack.last() {
                if parent.borrow().left.is_none() {
                    parent.borrow_mut().left = Some(node.clone());
                } else {
                    parent.borrow_mut().right = Some(node.clone());
                }
            }
            
            stack.push((level, node));
        }
        
        stack.first().map(|x| x.1.clone())
    }
}
