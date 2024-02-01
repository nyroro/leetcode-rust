
// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//   pub val: i32,
//   pub next: Option<Box<ListNode>>
// }
// 
// impl ListNode {
//   #[inline]
//   fn new(val: i32) -> Self {
//     ListNode {
//       next: None,
//       val

//     }
//   }
// }

impl Solution {
    pub fn delete_duplicates(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut current = head;
        
        while let Some(node) = current.as_mut() {
            if let Some(next_node) = node.next.as_mut() {
                if node.val == next_node.val {
                    node.next = next_node.next.take();
                } else {
                    current = node.next.take();
                }
            } else {
                break;
            }
        }
        
        head

    }
}
