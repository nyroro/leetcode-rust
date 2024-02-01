
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
    pub fn nodes_between_critical_points(head: Option<Box<ListNode>>) -> Vec<i32> {
        let mut arr: Vec<i32> = Vec::new();
        let mut p = &head;
        while let Some(node) = p {
            arr.push(node.val);
            p = &node.next;
        }
        
        let mut arr2: Vec<usize> = Vec::new();
        for i in 1..arr.len()-1 {
            if (arr[i-1] < arr[i] && arr[i+1] < arr[i]) || (arr[i-1] > arr[i] && arr[i+1] > arr[i]) {
                arr2.push(i);
            }
        }
        
        if arr2.len() <= 1 {
            return vec![-1, -1];
        }
        
        let y = (arr2[arr2.len()-1] - arr2[0]) as i32;
        let mut arr3: Vec<i32> = Vec::new();
        for i in 1..arr2.len() {
            arr3.push((arr2[i] - arr2[i-1]) as i32);
        }
        let x = *arr3.iter().min().unwrap();
        
        vec![x, y]
    }
}
