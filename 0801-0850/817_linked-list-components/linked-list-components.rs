
use std::collections::HashSet;

impl Solution {
    pub fn num_components(head: Option<Box<ListNode>>, nums: Vec<i32>) -> i32 {
        let mut set = nums.into_iter().collect::<HashSet<_>>();
        let mut count = 0;
        let mut current = &head;
        
        while let Some(node) = current {
            if set.contains(&node.val) && (node.next.is_none() || !set.contains(&node.next.as_ref().unwrap().val)) {
                count += 1;
            }
            current = &node.next;
        }
        
        count

    }
}
