
impl Solution {
    pub fn soup_servings(n: i32) -> f64 {
        if n >= 5000 {
            return 1.0;
        }
        
        let n = (n + 24) / 25;
        let mut dp = vec![vec![0.0; n as usize + 1]; n as usize + 1];
        
        dp[0][0] = 0.5;
        
        for i in 1..=n as usize {
            dp[0][i] = 1.0;
        }
        
        for i in 1..=n as usize {
            for j in 1..=n as usize {
                dp[i][j] = 0.25 * (dp[(i as i32 - 4).max(0) as usize][j] + dp[(i as i32 - 3).max(0) as usize][(j as i32 - 1).max(0) as usize] + dp[(i as i32 - 2).max(0) as usize][(j as i32 - 2).max(0) as usize] + dp[(i as i32 - 1).max(0) as usize][(j as i32 - 3).max(0) as usize]);
            }
        }
        
        dp[n as usize][n as usize]
    }
}
