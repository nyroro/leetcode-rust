
impl Solution {
    pub fn sort_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        // Convert the linked list into a vector

        let mut arr = Vec::new();
        let mut p = head;
        while let Some(mut node) = p {
            let next_node = node.next.take();
            arr.push(node);
            p = next_node;
        }

        // Sort the vector using merge sort

        arr.sort_by(|a, b| a.val.cmp(&b.val));

        // Reconstruct the sorted linked list

        let mut dummy = ListNode::new(0);
        let mut current = &mut dummy;
        for node in arr {
            current.next = Some(node);
            current = current.next.as_mut().unwrap();
        }
        current.next = None;

        // Return the head of the sorted linked list

        dummy.next

    }
}
