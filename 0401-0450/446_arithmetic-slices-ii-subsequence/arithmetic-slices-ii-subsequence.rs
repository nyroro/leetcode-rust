
use std::collections::HashMap;

impl Solution {
    pub fn number_of_arithmetic_slices(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut count = 0;
        let mut dp = vec![HashMap::new(); n];
        
        for i in 0..n {
            for j in 0..i {
                let diff = nums[i] as i64 - nums[j] as i64;
                let sum = *dp[j].get(&diff).unwrap_or(&0);
                let original = *dp[i].get(&diff).unwrap_or(&0);
                dp[i].insert(diff, original + sum + 1);
                count += sum;
            }
        }
        
        count

    }
}
