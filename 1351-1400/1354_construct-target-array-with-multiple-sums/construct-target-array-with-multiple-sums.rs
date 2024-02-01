
use std::collections::BinaryHeap;

impl Solution {
    pub fn is_possible(target: Vec<i32>) -> bool {
        let mut max_heap: BinaryHeap<i32> = BinaryHeap::new();
        let mut total: i32 = 0;
        
        for &num in target.iter() {
            max_heap.push(num);
            total += num;
        }
        
        while let Some(max) = max_heap.pop() {
            total -= max;
            if max == 1 || total == 1 {
                return true;
            }
            if max < total || total == 0 || max % total == 0 {
                return false;
            }
            max %= total;
            total += max;
            max_heap.push(max);
        }
        
        false

    }
}
