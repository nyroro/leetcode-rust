
impl Solution {
    pub fn is_palindrome(head: Option<Box<ListNode>>) -> bool {
        let mut slow = head.as_ref();
        let mut fast = head.as_ref();
        
        // Find the middle of the linked list

        while fast.is_some() && fast.as_ref().unwrap().next.is_some() {
            slow = slow.unwrap().next.as_ref();
            fast = fast.unwrap().next.as_ref().unwrap().next.as_ref();
        }
        
        // Reverse the second half of the linked list

        let mut prev = None;
        let mut curr = slow;
        while let Some(mut node) = curr {
            let next = node.next.take();
            node.next = prev.take();
            prev = Some(node);
            curr = next;
        }
        
        // Compare the first and second half of the linked list

        let mut first = head.as_ref();
        let mut second = prev.as_ref();
        while first.is_some() && second.is_some() {
            if first.unwrap().val != second.unwrap().val {
                return false;
            }
            first = first.unwrap().next.as_ref();
            second = second.unwrap().next.as_ref();
        }
        
        true

    }
}
