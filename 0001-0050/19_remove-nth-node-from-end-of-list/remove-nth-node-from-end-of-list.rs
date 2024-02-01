impl Solution {
    pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        let mut dummy = Some(Box::new(ListNode::new(0)));
        dummy.as_mut().unwrap().next = head;
        let length = Self::get_length(dummy.as_ref(), 0);
        Self::remove_nth(&mut dummy, length - n, 0);
        dummy.unwrap().next
    }
    
    fn get_length(node: Option<&Box<ListNode>>, count: i32) -> i32 {
        match node {
            Some(n) => Self::get_length(n.next.as_ref(), count + 1),
            None => count,
        }
    }
    
    fn remove_nth(node: &mut Option<Box<ListNode>>, target: i32, count: i32) {
        if let Some(n) = node {
            if count == target {
                *node = n.next.take();
            } else {
                Self::remove_nth(&mut n.next, target, count + 1);
            }
        }
    }
}
