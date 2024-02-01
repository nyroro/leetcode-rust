
impl Solution {
    pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let (mut l1_current, mut l2_current) = (l1.as_ref(), l2.as_ref());
        let (mut carry, mut dummy) = (0, ListNode::new(0));
        let mut current = &mut dummy;

        while l1_current.is_some() || l2_current.is_some() {
            let mut sum = carry;
            if let Some(node) = l1_current {
                sum += node.val;
                l1_current = node.next.as_ref();
            }
            if let Some(node) = l2_current {
                sum += node.val;
                l2_current = node.next.as_ref();
            }
            carry = sum / 10;
            current.next = Some(Box::new(ListNode::new(sum % 10)));
            current = current.next.as_mut().unwrap();
        }

        if carry > 0 {
            current.next = Some(Box::new(ListNode::new(carry)));
        }

        dummy.next

    }
}
