
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
    pub fn sort_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        // Convert the linked list into a vector

        let mut arr = Vec::new();
        let mut p = head;
        while let Some(node) = p {
            arr.push(node);
            p = node.next;
        }

        // Sort the vector using merge sort

        arr.sort_by(|a, b| a.val.cmp(&b.val));

        // Update the next pointers to reflect the sorted order

        for i in 0..arr.len() - 1 {
            arr[i].next = Some(arr[i + 1].clone());
        }
        arr.last().map(|last| last.next = None);

        // Return the head of the sorted linked list

        arr.first().map(|first| Some(first.clone()))
    }
}
