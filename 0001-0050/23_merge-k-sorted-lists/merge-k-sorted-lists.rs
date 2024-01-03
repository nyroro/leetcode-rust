
impl Solution {
    pub fn merge_k_lists(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
        fn merge_two_lists(
            l1: Option<Box<ListNode>>,
            l2: Option<Box<ListNode>>,
        ) -> Option<Box<ListNode>> {
            match (l1, l2) {
                (Some(mut l), Some(mut r)) => {
                    if l.val < r.val {
                        l.next = merge_two_lists(l.next, Some(r));
                        Some(l)
                    } else {
                        r.next = merge_two_lists(Some(l), r.next);
                        Some(r)
                    }
                }
                (Some(l), None) => Some(l),
                (None, Some(r)) => Some(r),
                _ => None,
            }
        }
        
        let n = lists.len();
        if n == 0 {
            return None;
        } else if n == 1 {
            return lists[0].clone();
        }
        
        let mid = n / 2;
        let left = Solution::merge_k_lists(lists[..mid].to_vec());
        let right = Solution::merge_k_lists(lists[mid..].to_vec());
        
        merge_two_lists(left, right)
    }
}
