
use std::collections::BinaryHeap;
use std::collections::HashMap;

impl Solution {
    pub fn min_length_after_removals(nums: Vec<i32>) -> i32 {
        if nums.len() == 1 {
            return 1;
        }
        
        let mut counter = HashMap::new();
        for &num in &nums {
            *counter.entry(num).or_insert(0) += 1;
        }
        
        let mut heap = BinaryHeap::new();
        for &count in counter.values() {
            heap.push(count);
        }
        
        while heap.len() > 1 {
            let t1 = heap.pop().unwrap();
            let t2 = heap.pop().unwrap();
            if heap.is_empty() {
                let t = t1 - t2;
                heap.push(t);
            } else {
                let t3 = heap.peek().unwrap();
                let t = t2 - t3 + 1;
                let t1 = t1 - t;
                let t2 = t2 - t;
                if t1 > 0 {
                    heap.push(t1);
                }
                if t2 > 0 {
                    heap.push(t2);
                }
            }
        }
        
        return heap.pop().unwrap();
    }
}
