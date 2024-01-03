
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
    pub fn pair_sum(head: Option<Box<ListNode>>) -> i32 {
        // 创建一个可变的数组来存储链表节点的值

        let mut values = Vec::new();
        let mut current = head;

        // 遍历链表并将节点的值存储在数组中

        while let Some(node) = current {
            values.push(node.val);
            current = node.next;
        }

        // 使用双指针技术找到每对节点及其孪生节点，并计算它们的和

        let (mut left, mut right) = (0, values.len() - 1);
        let mut max_twin_sum = 0;

        while left < right {
            let twin_sum = values[left] + values[right];
            max_twin_sum = max_twin_sum.max(twin_sum);
            left += 1;
            right -= 1;
        }

        // 返回最大的孪生节点和作为结果

        max_twin_sum

    }
}
