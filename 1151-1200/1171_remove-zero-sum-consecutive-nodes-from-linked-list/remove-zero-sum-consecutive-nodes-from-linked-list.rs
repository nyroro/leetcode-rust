
impl Solution {
    pub fn remove_zero_sum_sublists(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut vec: Vec<i32> = Vec::new();
        let mut temp = &head;
        while let Some(node) = temp {
            vec.push(node.val);
            temp = &node.next;
        }
        
        for start in 0..vec.len() {
            let mut sum = 0;
            for end in start..vec.len() {
                sum += vec[end];
                if sum == 0 {
                    for i in start..=end {
                        vec[i] = 0;
                    }
                    break;
                }
            }
        }
        
        let mut alt = ListNode::new(0);
        let mut a = &mut alt;
        for &val in &vec {
            if val != 0 {
                a.next = Some(Box::new(ListNode::new(val)));
                a = a.next.as_mut().unwrap();
            }
        }
        a.next = None;
        
        if let Some(node) = alt.next {
            Some(node)
        } else {
            None

        }
    }
}
