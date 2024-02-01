
impl Solution {
    pub fn min_difficulty(job_difficulty: Vec<i32>, d: i32) -> i32 {
        let n = job_difficulty.len();
        if n < d as usize {
            return -1;
        }
        
        let mut dp = vec![vec![i32::MAX; n + 1]; d as usize + 1];
        dp[0][0] = 0;
        
        for i in 1..=d as usize {
            for j in i..=n {
                let mut max_difficulty = 0;
                for k in (i - 1..j).rev() {
                    max_difficulty = max_difficulty.max(job_difficulty[k]);
                    dp[i][j] = dp[i][j].min(dp[i - 1][k] + max_difficulty);
                }
            }
        }
        
        if dp[d as usize][n] == i32::MAX {
            return -1;
        } else {
            return dp[d as usize][n];
        }
    }
}
