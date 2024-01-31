
use std::collections::BinaryHeap;
use std::cmp::Reverse;



impl Solution {
    pub fn max_spending(values: Vec<Vec<i32>>) -> i64 {
        let mut flattened_values: Vec<i32> = values.into_iter().flatten().collect();
        flattened_values.sort(); // Sort the values in non-decreasing order


        let mut heap = BinaryHeap::new();
        for val in flattened_values {
            heap.push(Reverse(val));
        }

        let mut ans = 0;
        for i in 1..=heap.len() {
            if let Some(Reverse(val)) = heap.pop() {
                ans += (i as i64) * (val as i64);
            }
        }

        ans

    }
}
