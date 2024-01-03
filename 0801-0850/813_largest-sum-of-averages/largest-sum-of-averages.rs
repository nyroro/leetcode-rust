
impl Solution {
    pub fn largest_sum_of_averages(nums: Vec<i32>, k: i32) -> f64 {
        let n = nums.len();
        let mut dp = vec![vec![0.0; n + 1]; k as usize + 1];
        let mut sums = vec![0; n + 1];
        
        for i in 0..n {
            sums[i + 1] = sums[i] + nums[i];
            dp[1][i + 1] = sums[i + 1] as f64 / (i + 1) as f64;
        }
        
        for i in 2..=k as usize {
            for j in i..=n {
                for l in (i - 1)..j {
                    dp[i][j] = dp[i][j].max(dp[i - 1][l] + (sums[j] - sums[l]) as f64 / (j - l) as f64);
                }
            }
        }
        
        dp[k as usize][n]
    }
}
