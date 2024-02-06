
use std::collections::HashMap;

impl Solution {
    pub fn longest_square_streak(nums: Vec<i32>) -> i32 {
        let mut nums = nums;
        nums.sort();
        let mut streak_lengths = HashMap::new();
        let mut longest_streak = 0;

        for &n in nums.iter() {
            let sqrt = (n as f64).sqrt() as i32;
            if sqrt * sqrt == n {
                let prev_streak = *streak_lengths.get(&sqrt).unwrap_or(&0);
                let current_streak = prev_streak + 1;
                streak_lengths.insert(n, current_streak);
                longest_streak = longest_streak.max(current_streak);
            } else {
                streak_lengths.insert(n, 1);
            }
        }

        if longest_streak > 1 {
            longest_streak

        } else {
            -1

        }
    }
}
