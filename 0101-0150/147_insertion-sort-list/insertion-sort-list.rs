
impl Solution {
    pub fn insertion_sort_list(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut dummy = Box::new(ListNode { val: 0, next: None });
        let mut curr = head;
        
        while let Some(mut node) = curr {
            curr = node.next.take();
            let mut prev = &mut dummy;
            while prev.next.as_ref().map_or(false, |next| next.val < node.val) {
                prev = prev.next.as_mut().unwrap();
            }
            node.next = prev.next.take();
            prev.next = Some(node);
        }
        
        dummy.next

    }
}
