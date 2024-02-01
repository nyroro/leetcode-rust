
impl Solution {
    pub fn min_side_jumps(obstacles: Vec<i32>) -> i32 {
        let n = obstacles.len();
        let mut dp = vec![vec![0; 3]; n];
        
        dp[0][0] = 1;
        dp[0][2] = 1;
        
        for i in 1..n {
            if obstacles[i] != 1 {
                dp[i][0] = dp[i-1][0];
            }
            if obstacles[i] != 2 {
                dp[i][1] = dp[i-1][1];
            }
            if obstacles[i] != 3 {
                dp[i][2] = dp[i-1][2];
            }
            
            if obstacles[i] != 1 {
                dp[i][0] = dp[i][0].min(dp[i][1] + 1).min(dp[i][2] + 1);
            }
            if obstacles[i] != 2 {
                dp[i][1] = dp[i][1].min(dp[i][0] + 1).min(dp[i][2] + 1);
            }
            if obstacles[i] != 3 {
                dp[i][2] = dp[i][2].min(dp[i][0] + 1).min(dp[i][1] + 1);
            }
        }
        
        dp[n-1][0].min(dp[n-1][1].min(dp[n-1][2])) as i32

    }
}
