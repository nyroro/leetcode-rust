
use std::collections::BinaryHeap;

impl Solution {
    pub fn min_stone_sum(piles: Vec<i32>, k: i32) -> i32 {
        let mut max_heap = BinaryHeap::new();
        let mut sum = 0;

        for pile in piles {
            max_heap.push(pile);
            sum += pile;
        }

        for _ in 0..k {
            if let Some(max_pile) = max_heap.pop() {
                let removed_stones = max_pile / 2;
                sum -= removed_stones;
                max_heap.push(max_pile - removed_stones);
            }
        }

        sum

    }
}
