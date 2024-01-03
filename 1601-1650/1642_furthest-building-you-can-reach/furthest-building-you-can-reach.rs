
use std::collections::BinaryHeap;
use std::cmp::Reverse;

impl Solution {
    pub fn furthest_building(heights: Vec<i32>, bricks: i32, ladders: i32) -> i32 {
        let mut pq = BinaryHeap::new();
        let n = heights.len();
        let mut bricks = bricks;

        for i in 0..n-1 {
            let diff = heights[i+1] - heights[i];
            if diff > 0 {
                pq.push(Reverse(diff));
            }
            if pq.len() > ladders as usize {
                if let Some(Reverse(d)) = pq.pop() {
                    if bricks >= d {
                        bricks -= d;
                    } else {
                        return i as i32;
                    }
                }
            }
        }
        n as i32 - 1

    }
}
