
impl Solution {
    fn reverse_k_group(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        let mut current = &head;
        let mut count = 0;

        // Count the number of nodes in the current group

        while count < k && current.is_some() {
            current = &current.as_ref().unwrap().next;
            count += 1;
        }

        // If the current group has k nodes, reverse it

        if count == k {
            let mut prev = None;
            let mut next = head;

            for _ in 0..k {
                let mut temp = next.as_mut().unwrap().next.take();
                next.as_mut().unwrap().next = prev.take();
                prev = next;
                next = temp;
            }

            // Recursively reverse the remaining nodes

            let new_next = Self::reverse_k_group(next, k);

            let mut current = &mut prev;
            while current.as_mut().unwrap().next.is_some() {
                current = &mut current.as_mut().unwrap().next;
            }
            current.as_mut().unwrap().next = new_next;

            prev

        } else {
            head

        }
    }
}
