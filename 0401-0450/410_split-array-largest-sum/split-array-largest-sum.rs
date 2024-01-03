
impl Solution {
    pub fn split_array(nums: Vec<i32>, k: i32) -> i32 {
        let n = nums.len();
        let mut dp = vec![vec![std::i32::MAX; k as usize + 1]; n + 1];
        let mut prefix_sum = vec![0; n + 1];
        
        for i in 1..=n {
            prefix_sum[i] = prefix_sum[i - 1] + nums[i - 1];
        }
        
        dp[0][0] = 0;
        
        for i in 1..=n {
            for j in 1..=k as usize {
                for m in (j - 1)..i {
                    let sum = prefix_sum[i] - prefix_sum[m];
                    dp[i][j] = dp[i][j].min(dp[m][j - 1].max(sum));
                }
            }
        }
        
        dp[n][k as usize]
    }
}
