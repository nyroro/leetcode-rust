use std::collections::BinaryHeap;
use std::cmp::Reverse;

impl Solution {
    pub fn maximum_product(nums: Vec<i32>, k: i32) -> i32 {
        let mut heap = BinaryHeap::new();
        
        for &num in nums.iter() {
            heap.push(Reverse(num));
        }
        
        for _ in 0..k {
            if let Some(Reverse(max)) = heap.pop() {
                heap.push(Reverse(max + 1));
            }
        }
        
        let modulo = 1_000_000_007;
        let mut result = 1_i64;
        while let Some(Reverse(num)) = heap.pop() {
            result = (result * num as i64) % modulo;
        }
        
        result as i32

    }
}
