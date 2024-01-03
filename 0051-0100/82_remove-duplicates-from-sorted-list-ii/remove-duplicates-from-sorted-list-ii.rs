
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
        use std::collections::HashMap;

        // 辅助函数：打印链表

        fn print_list(head: &Option<Box<ListNode>>) {
            let mut current = head;
            while let Some(node) = current {
                print!("{} ", node.val);
                current = &node.next;
            }
            println!();
        }

        // 构建哈希表记录每个节点值的出现次数

        let mut count_map = HashMap::new();
        let mut current = &head;
        while let Some(node) = current {
            *count_map.entry(node.val).or_insert(0) += 1;
            current = &node.next;
        }

        // 构建新链表，只保留出现次数为1的节点

        let mut dummy = Some(Box::new(ListNode { val: 0, next: head }));
        let mut prev = &mut dummy;
        let mut current = &mut prev.as_mut().unwrap().next;
        while let Some(node) = current {
            if let Some(&count) = count_map.get(&node.val) {
                if count == 1 {
                    prev = &mut prev.as_mut().unwrap().next;
                } else {
                    prev.as_mut().unwrap().next = node.next.take();
                }
            }
            current = &mut prev.as_mut().unwrap().next;
        }

        dummy.unwrap().next

    }
}
