
impl Solution {
    pub fn remove_elements(head: Option<Box<ListNode>>, val: i32) -> Option<Box<ListNode>> {
        if head.is_none() {
            return None;
        }
        
        let mut dummy = Some(Box::new(ListNode {
            val: 0,
            next: head,
        }));
        
        let mut prev = &mut dummy;
        let mut curr = &mut prev.as_mut().unwrap().next;
        
        while curr.is_some() {
            if curr.as_ref().unwrap().val == val {
                let next_node = curr.as_mut().unwrap().next.take();
                prev.as_mut().unwrap().next = next_node;
            } else {
                prev = &mut prev.as_mut().unwrap().next;
            }
            curr = &mut prev.as_mut().unwrap().next;
        }
        
        dummy.unwrap().next

    }
}
