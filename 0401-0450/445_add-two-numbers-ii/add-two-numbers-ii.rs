
impl Solution {
    pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut stack1 = Vec::new();
        let mut stack2 = Vec::new();
        
        let (mut l1_current, mut l2_current) = (l1.as_ref(), l2.as_ref());
        
        while let Some(node) = l1_current {
            stack1.push(node.val);
            l1_current = node.next.as_ref();
        }
        
        while let Some(node) = l2_current {
            stack2.push(node.val);
            l2_current = node.next.as_ref();
        }
        
        let mut carry = 0;
        let mut result = None;
        
        while !stack1.is_empty() || !stack2.is_empty() || carry > 0 {
            let mut sum = carry;
            if let Some(val) = stack1.pop() {
                sum += val;
            }
            if let Some(val) = stack2.pop() {
                sum += val;
            }
            carry = sum / 10;
            let mut new_node = ListNode::new(sum % 10);
            new_node.next = result;
            result = Some(Box::new(new_node));
        }
        
        result

    }
}
