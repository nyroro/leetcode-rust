
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
    pub fn split_list_to_parts(head: Option<Box<ListNode>>, k: i32) -> Vec<Option<Box<ListNode>>> {
        // 计算链表的长度

        let mut len = 0;
        let mut node = &head;
        while let Some(n) = node {
            len += 1;
            node = &n.next;
        }
        
        // 计算基本长度和余数

        let base_len = len / k;
        let remainder = len % k;
        
        // 分割链表

        let mut result = Vec::new();
        let mut node = head;
        for i in 0..k {
            let mut part_len = base_len;
            if i < remainder {
                part_len += 1;
            }
            
            let mut part_head = node.take();
            let mut part_tail = &mut part_head;
            for _ in 1..part_len {
                if let Some(n) = part_tail {
                    part_tail = &mut n.next;
                }
            }
            
            if let Some(n) = part_tail {
                node = n.next.take();
            }
            
            result.push(part_head);
        }
        
        result

    }
}
