
use std::collections::BinaryHeap;

impl Solution {
    pub fn earliest_full_bloom(plant_time: Vec<i32>, grow_time: Vec<i32>) -> i32 {
        let mut ret = plant_time.iter().sum::<i32>();
        let mut arr: Vec<(i32, i32)> = plant_time.iter().zip(grow_time.iter()).map(|(&p, &g)| (-g, p)).collect();
        
        arr.sort(); // Sort the array to make it a min-heap


        let mut now = 0;
        for (g, p) in arr {
            let g = -g;
            now += p;
            ret = ret.max(now + g);
        }

        ret

    }
}
