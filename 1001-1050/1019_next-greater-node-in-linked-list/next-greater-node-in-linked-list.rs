
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
    pub fn next_larger_nodes(head: Option<Box<ListNode>>) -> Vec<i32> {
        let mut stack: Vec<(i32, usize)> = Vec::new();
        let mut result: Vec<i32> = Vec::new();
        let mut index = 0;

        let mut current = head;
        while let Some(node) = current {
            let val = node.val;
            result.push(0);

            while let Some((prev_val, prev_index)) = stack.pop() {
                if prev_val < val {
                    result[prev_index] = val;
                } else {
                    stack.push((prev_val, prev_index));
                    break;
                }
            }

            stack.push((val, index));
            index += 1;
            current = node.next;
        }

        result

    }
}
