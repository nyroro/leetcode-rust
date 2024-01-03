
impl Solution {
    pub fn partition(head: Option<Box<ListNode>>, x: i32) -> Option<Box<ListNode>> {
        let mut dummy1 = Some(Box::new(ListNode::new(0)));
        let mut dummy2 = Some(Box::new(ListNode::new(0)));
        let mut less_ptr = &mut dummy1;
        let mut greater_ptr = &mut dummy2;
        
        let mut curr = head;
        while let Some(mut node) = curr {
            curr = node.next.take();
            if node.val < x {
                less_ptr.as_mut().unwrap().next = Some(node);
                less_ptr = &mut less_ptr.as_mut().unwrap().next;
            } else {
                greater_ptr.as_mut().unwrap().next = Some(node);
                greater_ptr = &mut greater_ptr.as_mut().unwrap().next;
            }
        }
        
        less_ptr.as_mut().unwrap().next = dummy2.unwrap().next;
        dummy1.unwrap().next

    }
}
