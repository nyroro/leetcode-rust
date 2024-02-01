
impl Solution {
    pub fn is_palindrome(head: Option<Box<ListNode>>) -> bool {
        let mut slow = head.as_ref();
        let mut fast = head.as_ref();
        
        // 找到链表的中间节点

        while fast.is_some() && fast.as_ref().unwrap().next.is_some() {
            slow = slow.unwrap().next.as_ref();
            fast = fast.unwrap().next.as_ref().unwrap().next.as_ref();
        }
        
        // 反转链表的后半部分

        let mut prev = None;
        let mut curr = slow.cloned();
        while let Some(mut node) = curr {
            let next = node.next.take();
            node.next = prev.take();
            prev = Some(node);
            curr = next;
        }
        
        // 比较链表的前半部分和反转后的后半部分是否相等

        let mut first = head.as_ref();
        let mut second = prev.as_ref();
        while first.is_some() && second.is_some() {
            if first.unwrap().val != second.unwrap().val {
                return false;
            }
            first = first.unwrap().next.as_ref();
            second = second.unwrap().next.as_ref();
        }
        
        true

    }
}
