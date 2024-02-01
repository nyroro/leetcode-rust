
use std::collections::BinaryHeap;

impl Solution {
    pub fn earliest_full_bloom(plant_time: Vec<i32>, grow_time: Vec<i32>) -> i32 {
        let mut heap = BinaryHeap::new();
        let mut now = 0;
        let mut ret = 0;

        for (p, g) in plant_time.iter().zip(grow_time.iter()) {
            heap.push((-g, p));
        }

        while let Some((g, p)) = heap.pop() {
            let g = -g;
            now += p;
            ret = ret.max(now + g);
        }

        ret

    }
}
