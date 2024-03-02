
impl Solution {
    pub fn delete_middle(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        if head.is_none() || head.as_ref().unwrap().next.is_none() {
            return None;
        }

        let mut slow = &head;
        let mut fast = &head;

        let mut prev: *mut Option<Box<ListNode>> = std::ptr::null_mut();

        while fast.is_some() && fast.as_ref().unwrap().next.is_some() {
            prev = slow as *const _ as *mut _;
            slow = &slow.as_ref().unwrap().next;
            fast = &fast.as_ref().unwrap().next.as_ref().unwrap().next;
        }

        if !prev.is_null() {
            let prev = unsafe { &mut *prev };
            if let Some(node) = prev {
                let next = node.next.as_mut().unwrap().next.take();
                node.next = next;
            }
        }

        head

    }
}
