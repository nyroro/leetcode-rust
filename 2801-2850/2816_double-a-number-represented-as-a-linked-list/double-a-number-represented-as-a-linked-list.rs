
impl Solution {
    pub fn double_it(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut prev = None;
        let mut curr = head;
        let mut rem = 0;
        let mut list = Some(Box::new(ListNode { val: 0, next: None }));

        while let Some(mut node) = curr {
            let next = node.next.take();
            node.next = prev;
            prev = Some(node);
            curr = next;
        }

        let mut temp = prev;
        while let Some(mut node) = temp {
            let total = node.val * 2 + rem;
            list.as_mut().unwrap().val = total % 10;
            rem = total / 10;
            let res = Some(Box::new(ListNode { val: rem, next: list }));
            list = res;
            temp = node.next.take().map(|n| Box::new(*n));
        }

        if rem > 0 {
            list

        } else {
            list.unwrap().next.take()
        }
    }
}
