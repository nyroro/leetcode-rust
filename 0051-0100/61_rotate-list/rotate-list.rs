
impl Solution {
    pub fn rotate_right(mut head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        // Handle the base cases

        if head.is_none() || head.as_ref()?.next.is_none() {
            return head;
        }
        
        // Find the length of the linked list and the last node

        let mut len = 1;
        let mut last_node = head.as_mut().unwrap();
        while let Some(node) = last_node.next.as_mut() {
            len += 1;
            last_node = node;
        }
        
        // Calculate the actual number of places to rotate

        let k = k % len;
        if k == 0 {
            return head;
        }
        
        // Find the new last node after rotation

        let mut new_last_node = head.as_mut().unwrap();
        for _ in 0..len - k - 1 {
            new_last_node = new_last_node.next.as_mut().unwrap();
        }
        
        // Perform the rotation

        let mut new_head = new_last_node.next.take();
        let mut current = &mut new_head;
        while current.as_ref()?.next.is_some() {
            current = &mut current.as_mut()?.next;
        }
        current.as_mut()?.next = head;
        
        new_head

    }
}
