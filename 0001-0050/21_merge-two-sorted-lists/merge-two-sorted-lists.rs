
impl Solution {
    pub fn merge_two_lists(list1: Option<Box<ListNode>>, list2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut dummy_head = Some(Box::new(ListNode::new(0)));
        let mut curr = &mut dummy_head;

        let mut p1 = list1;
        let mut p2 = list2;

        while let (Some(node1), Some(node2)) = (p1.as_ref(), p2.as_ref()) {
            if node1.val <= node2.val {
                let next = p1.as_mut().unwrap().next.take();
                curr.as_mut().unwrap().next = p1.take();
                p1 = next;
            } else {
                let next = p2.as_mut().unwrap().next.take();
                curr.as_mut().unwrap().next = p2.take();
                p2 = next;
            }
            curr = &mut curr.as_mut().unwrap().next;
        }

        if p1.is_some() {
            curr.as_mut().unwrap().next = p1;
        } else {
            curr.as_mut().unwrap().next = p2;
        }

        dummy_head.unwrap().next

    }
}
