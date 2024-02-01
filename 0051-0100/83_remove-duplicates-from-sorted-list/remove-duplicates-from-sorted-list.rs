
impl Solution {
    pub fn delete_duplicates(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut dummy = Some(Box::new(ListNode { val: 0, next: head }));
        let mut current = dummy.as_mut();

        while let Some(node) = current.as_mut().unwrap().next.as_mut() {
            if let Some(next_node) = node.next.as_mut() {
                if node.val == next_node.val {
                    node.next = next_node.next.take();
                } else {
                    current = current.unwrap().next.as_mut();
                }
            } else {
                break;
            }
        }

        dummy.unwrap().next

    }
}
