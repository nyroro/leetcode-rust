
impl Solution {
    pub fn min_difficulty(job_difficulty: Vec<i32>, d: i32) -> i32 {
        let n = job_difficulty.len();
        if n < d as usize {
            return -1;
        }
        
        let mut dp = vec![vec![0; n]; d as usize];
        dp[0][0] = job_difficulty[0];
        for i in 1..n {
            dp[0][i] = job_difficulty[i].max(dp[0][i - 1]);
        }
        
        for i in 1..d as usize {
            for j in i..n {
                let mut max_difficulty = job_difficulty[j];
                dp[i][j] = i32::MAX;
                for k in (i..=j).rev() {
                    max_difficulty = max_difficulty.max(job_difficulty[k]);
                    dp[i][j] = dp[i][j].min(dp[i - 1][k - 1] + max_difficulty);
                }
            }
        }
        
        if dp[d as usize - 1][n - 1] == i32::MAX {
            return -1;
        } else {
            return dp[d as usize - 1][n - 1];
        }
    }
}
