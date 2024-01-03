
impl Solution {
    pub fn two_egg_drop(n: i32) -> i32 {
        let n = n as usize;
        let mut dp = vec![vec![0; 3]; n + 1];
        
        for i in 1..=n {
            dp[i][1] = i as i32;
        }
        
        for j in 1..=2 {
            dp[1][j] = 1;
        }
        
        for i in 2..=n {
            for j in 2..=2 {
                dp[i][j] = i as i32;
                for k in 1..=i {
                    dp[i][j] = dp[i][j].min(1 + dp[k-1][j-1].max(dp[i-k][j]));
                }
            }
        }
        
        dp[n][2]
    }
}
