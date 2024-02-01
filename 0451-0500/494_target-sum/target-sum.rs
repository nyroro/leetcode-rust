
impl Solution {
    pub fn find_target_sum_ways(nums: Vec<i32>, target: i32) -> i32 {
        let sum: i32 = nums.iter().sum();
        let diff = sum - target.abs() * 2;
        if diff < 0 || diff % 2 != 0 {
            return 0;
        }
        let neg = (diff / 2) as usize;
        let mut dp = vec![vec![0; neg + 1]; nums.len() + 1];
        dp[0][0] = 1;
        for i in 1..=nums.len() {
            for j in 0..=neg {
                dp[i][j] = dp[i - 1][j];
                if j >= nums[i - 1] as usize {
                    dp[i][j] += dp[i - 1][j - nums[i - 1] as usize];
                }
            }
        }
        dp[nums.len()][neg]
    }
}
