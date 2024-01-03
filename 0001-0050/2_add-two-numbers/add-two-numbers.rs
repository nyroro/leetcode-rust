
impl Solution {
    pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut l1_current = l1;
        let mut l2_current = l2;
        let mut dummy = ListNode::new(0);
        let mut current = &mut dummy;
        let mut carry = 0;
        
        while l1_current.is_some() || l2_current.is_some() {
            let mut sum = carry;
            
            if let Some(node) = l1_current {
                sum += node.val;
                l1_current = node.next;
            }
            
            if let Some(node) = l2_current {
                sum += node.val;
                l2_current = node.next;
            }
            
            carry = sum / 10;
            current.next = Some(Box::new(ListNode::new(sum % 10)));
            current = current.next.as_mut().unwrap();
        }
        
        if carry != 0 {
            current.next = Some(Box::new(ListNode::new(carry)));
        }
        
        dummy.next

    }
}
