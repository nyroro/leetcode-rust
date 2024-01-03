
use std::collections::BinaryHeap;

impl Solution {
    pub fn kth_smallest(matrix: Vec<Vec<i32>>, k: i32) -> i32 {
        let mut min_heap = BinaryHeap::new();
        
        for row in matrix {
            for num in row {
                min_heap.push(num);
                if min_heap.len() > k as usize {
                    min_heap.pop();
                }
            }
        }
        
        min_heap.pop().unwrap()
    }
}
