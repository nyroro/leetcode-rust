
use std::collections::HashMap;

impl Solution {
    pub fn num_subarrays_with_sum(nums: Vec<i32>, goal: i32) -> i32 {
        let mut prefix_sum = vec![0; nums.len() + 1];
        for i in 0..nums.len() {
            prefix_sum[i + 1] = prefix_sum[i] + nums[i];
        }
        
        let mut count = 0;
        let mut sum_count = HashMap::new();
        for i in 0..prefix_sum.len() {
            if let Some(&c) = sum_count.get(&(prefix_sum[i] - goal)) {
                count += c;
            }
            *sum_count.entry(prefix_sum[i]).or_insert(0) += 1;
        }
        
        count

    }
}
