
// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>
}

impl ListNode {
  #[inline]
  fn new(val: i32) -> Self {
    ListNode {
      next: None,
      val

    }
  }
}

impl Solution {
    pub fn remove_elements(head: Option<Box<ListNode>>, val: i32) -> Option<Box<ListNode>> {
        let mut dummy = Some(Box::new(ListNode::new(0)));
        dummy.as_mut().unwrap().next = head;
        let mut prev = &mut dummy;
        let mut curr = &mut prev.as_mut().unwrap().next;
        
        while curr.is_some() {
            if curr.as_ref().unwrap().val == val {
                let next_node = curr.as_mut().unwrap().next.take();
                prev.as_mut().unwrap().next = next_node;
            } else {
                prev = &mut prev.as_mut().unwrap().next;
            }
            curr = &mut prev.as_mut().unwrap().next;
        }
        
        dummy.unwrap().next

    }
}
