
use std::collections::BinaryHeap;
use std::cmp::Reverse;

impl Solution {
    pub fn max_kelements(nums: Vec<i32>, k: i32) -> i64 {
        let mut res: i64 = 0;
        let mut heap = BinaryHeap::new();

        for num in nums {
            heap.push(Reverse(-num));
        }

        for _ in 0..k {
            if let Some(Reverse(a)) = heap.pop() {
                res -= a as i64;
                let new_val = (-a as f64 / 3.0).ceil() as i32;
                heap.push(Reverse(-new_val));
            }
        }

        res

    }
}
