
use std::collections::BinaryHeap;

impl Solution {
    pub fn minimum_deviation(nums: Vec<i32>) -> i32 {
        let mut pq = BinaryHeap::new();
        let mut max_val = 0;
        
        for num in nums.iter() {
            let mut n = *num;
            if n % 2 == 1 {
                n *= 2;
            }
            pq.push(n);
            max_val = max_val.max(n);
        }
        
        let mut min_deviation = std::i32::MAX;
        
        while let Some(mut min) = pq.pop() {
            min_deviation = min_deviation.min(max_val - min);
            if min % 2 == 0 {
                max_val = max_val.min(min / 2);
                min *= 2;
                pq.push(min);
            } else {
                break;
            }
        }
        
        min_deviation

    }
}
