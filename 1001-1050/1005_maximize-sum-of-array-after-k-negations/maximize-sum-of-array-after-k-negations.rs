
use std::collections::BinaryHeap;

impl Solution {
    pub fn largest_sum_after_k_negations(nums: Vec<i32>, k: i32) -> i32 {
        let mut heap = BinaryHeap::from(nums);
        
        for _ in 0..k {
            if let Some(mut min) = heap.pop() {
                min *= -1;
                heap.push(min);
            }
        }
        
        heap.iter().sum()
    }
}
