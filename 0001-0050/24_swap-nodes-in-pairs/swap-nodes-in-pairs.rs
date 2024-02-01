
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
    pub fn swap_pairs(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        // 处理边界情况

        if head.is_none() || head.as_ref().unwrap().next.is_none() {
            return head;
        }
        
        // 获取第一个节点和剩余节点

        let mut first = head.unwrap();
        let mut second = first.next.take().unwrap();
        
        // 交换节点

        first.next = Solution::swap_pairs(second.next.take());
        second.next = Some(first);
        
        Some(second)
    }
}
