
impl Solution {
    pub fn reorder_list(head: &mut Option<Box<ListNode>>) {
        // Step 1: Create a mutable vector to store the nodes of the linked list

        let mut nodes = Vec::new();
        let mut curr = head.take();  // Take ownership of the head


        // Step 2: Traverse the linked list and push each node into the vector

        while let Some(mut node) = curr {
            let next = node.next.take();  // Take ownership of the next node

            nodes.push(node);
            curr = next;
        }

        // Step 3: Perform the reordering of the list

        let n = nodes.len();
        let mut i = 0;
        let mut new_head = ListNode::new(0);  // Create a new head node

        let mut curr = &mut new_head;

        while i < n / 2 {
            curr.next = Some(nodes[i].clone());  // Clone the node and set it as the next node

            curr = curr.next.as_mut().unwrap();
            curr.next = Some(nodes[n - i - 1].clone());  // Clone the node and set it as the next node

            curr = curr.next.as_mut().unwrap();
            i += 1;
        }

        if n % 2 != 0 {
            curr.next = Some(nodes[i].clone());  // Clone the node and set it as the next node

            curr = curr.next.as_mut().unwrap();
        }

        // Step 4: Set the last node's next to None to terminate the list

        curr.next = None;

        *head = new_head.next;  // Update the head with the reordered list

    }
}
