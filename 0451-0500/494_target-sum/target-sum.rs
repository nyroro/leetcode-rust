
use std::collections::HashMap;

impl Solution {
    pub fn find_target_sum_ways(nums: Vec<i32>, target: i32) -> i32 {
        let mut dp: HashMap<i32, i32> = HashMap::new();
        dp.insert(0, 1);
        for num in nums {
            let mut new_dp: HashMap<i32, i32> = HashMap::new();
            for (&sum, &count) in dp.iter() {
                let sum_plus = sum + num;
                let sum_minus = sum - num;
                *new_dp.entry(sum_plus).or_insert(0) += count;
                *new_dp.entry(sum_minus).or_insert(0) += count;
            }
            dp = new_dp;
        }
        *dp.get(&target).unwrap_or(&0)
    }
}
