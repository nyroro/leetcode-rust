
impl Solution {
    pub fn merge_two_lists(list1: Option<Box<ListNode>>, list2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut dummy_head = Some(Box::new(ListNode::new(0)));
        let mut curr = &mut dummy_head;

        let (mut p1, mut p2) = (list1, list2);

        while let (Some(node1), Some(node2)) = (p1.as_ref(), p2.as_ref()) {
            if node1.val <= node2.val {
                curr.as_mut().unwrap().next = p1;
                p1 = p1.unwrap().next;
            } else {
                curr.as_mut().unwrap().next = p2;
                p2 = p2.unwrap().next;
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
