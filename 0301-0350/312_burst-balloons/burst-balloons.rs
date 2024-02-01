
impl Solution {
    pub fn max_coins(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut dp = vec![vec![0; n]; n];

        for len in 1..=n {
            for i in 0..=n-len {
                let j = i + len - 1;
                for k in i..=j {
                    let left = if k > 0 { nums[k-1] } else { 1 };
                    let right = if k < n-1 { nums[k+1] } else { 1 };
                    let score = left * nums[k] * right;
                    let left_dp = if k > i { dp[i][k-1] } else { 0 };
                    let right_dp = if k < j { dp[k+1][j] } else { 0 };
                    dp[i][j] = dp[i][j].max(score + left_dp + right_dp);
                }
            }
        }

        dp[0][n-1]
    }
}
