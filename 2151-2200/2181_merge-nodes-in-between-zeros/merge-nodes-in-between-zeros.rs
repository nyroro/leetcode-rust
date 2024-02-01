
impl Solution {
    pub fn merge_nodes(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut dummy = ListNode::new(0);
        let mut tail = &mut dummy;
        let mut sum = 0;

        while let Some(mut node) = head.take() {
            if node.val == 0 {
                if sum > 0 {
                    tail.next = Some(Box::new(ListNode::new(sum)));
                    tail = tail.next.as_mut().unwrap();
                    sum = 0;
                }
            } else {
                sum += node.val;
            }
            head = node.next.take();
        }

        if sum > 0 {
            tail.next = Some(Box::new(ListNode::new(sum)));
        }

        dummy.next

    }
}
