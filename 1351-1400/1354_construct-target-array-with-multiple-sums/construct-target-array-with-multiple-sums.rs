
use std::collections::BinaryHeap;

impl Solution {
    pub fn is_possible(target: Vec<i32>) -> bool {
        if target.len() == 1 {
            return target[0] == 1;
        }
        
        let mut max_heap: BinaryHeap<i64> = BinaryHeap::new();
        let mut total: i64 = 0;
        
        for &num in target.iter() {
            max_heap.push(num as i64);
            total += num as i64;
        }
        
        while let Some(max) = max_heap.pop() {
            total -= max;
            if max == 1 || total == 1 {
                return true;
            }
            if max < total || total == 0 || max % total == 0 {
                return false;
            }
            let new_max = max % total;
            total += new_max;
            max_heap.push(new_max);
        }
        
        false

    }
}
