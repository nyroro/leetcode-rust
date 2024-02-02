
use std::collections::BinaryHeap;
use std::cmp::Reverse;

impl Solution {
    pub fn smallest_chair(times: Vec<Vec<i32>>, target_friend: i32) -> i32 {
        let mut available: Vec<i32> = (0..times.len() as i32).collect();
        let mut leave_times: Vec<(i32, i32)> = Vec::new();

        let mut sorted_times = times.clone();
        sorted_times.sort();

        for time in sorted_times {
            let arrival = time[0];
            let leaving = time[1];
            while let Some(&(leave, chair)) = leave_times.first() {
                if leave <= arrival {
                    leave_times.remove(0);
                    available.push(chair);
                    available.sort_unstable();
                } else {
                    break;
                }
            }
            if arrival == times[target_friend as usize][0] {
                return available[0];
            }
            let chair = available.remove(0);
            leave_times.push((leaving, chair));
            leave_times.sort_by_key(|x| x.0);
        }

        -1

    }
}
