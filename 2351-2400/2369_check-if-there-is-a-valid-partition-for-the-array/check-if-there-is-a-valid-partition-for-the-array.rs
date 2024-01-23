
impl Solution {
    pub fn valid_partition(nums: Vec<i32>) -> bool {
        let mut dp = vec![false; nums.len() + 1];
        dp[0] = true;

        for i in 0..nums.len() {
            if i + 1 < nums.len() && nums[i] == nums[i + 1] {
                dp[i + 2] |= dp[i];
                if i + 2 < nums.len() && nums[i] == nums[i + 2] {
                    dp[i + 3] |= dp[i];
                }
            }
            if i + 2 < nums.len() && nums[i + 1] == nums[i] + 1 && nums[i + 2] == nums[i] + 2 {
                dp[i + 3] |= dp[i];
            }
        }
        dp[nums.len()]
    }
}
