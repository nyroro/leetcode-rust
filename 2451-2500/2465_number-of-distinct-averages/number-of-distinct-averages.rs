
use std::collections::HashSet;



impl Solution {
    pub fn distinct_averages(nums: Vec<i32>) -> i32 {
        let mut averages = HashSet::new();
        let mut sorted_nums = nums;
        sorted_nums.sort();

        for i in 0..(sorted_nums.len() / 2) {
            let sum = sorted_nums[i] + sorted_nums[sorted_nums.len() - i - 1];
            averages.insert(sum);
        }

        averages.len() as i32

    }
}
