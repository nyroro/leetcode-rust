
struct Solution {
    head: Option<Box<ListNode>>,
}

impl Solution {
    fn new(head: Option<Box<ListNode>>) -> Self {
        Solution { head }
    }
    
    fn get_random(&self) -> i32 {
        use rand::prelude::*;
        let mut rng = thread_rng();
        let mut result = 0;
        let mut count = 0;
        let mut curr = &self.head;
        
        while let Some(node) = curr {
            count += 1;
            if rng.gen_range(0, count) == 0 {
                result = node.val;
            }
            curr = &node.next;
        }
        
        result

    }
}
