
impl Solution {
    pub fn min_space_wasted_k_resizing(nums: Vec<i32>, k: i32) -> i32 {
        let n = nums.len();
        let mut dp = vec![vec![0; k as usize + 1]; n];
        let mut max_val = 0;
        let mut sum = 0;
        
        for i in 0..n {
            max_val = max_val.max(nums[i]);
            sum += nums[i];
            dp[i][0] = max_val * (i as i32 + 1) - sum;
        }
        
        for j in 1..=k as usize {
            for i in 0..n {
                let mut max_val = 0;
                let mut sum = 0;
                dp[i][j] = std::i32::MAX;
                for l in (0..=i).rev() {
                    max_val = max_val.max(nums[l]);
                    sum += nums[l];
                    dp[i][j] = if l > 0 {
                        dp[i][j].min(dp[l - 1][j - 1] + max_val * ((i - l + 1) as i32) - sum)
                    } else {
                        dp[i][j].min(max_val * ((i - l + 1) as i32) - sum)
                    };
                }
            }
        }
        
        dp[n - 1][k as usize]
    }
}
