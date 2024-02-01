
impl Solution {
    pub fn selling_wood(m: i32, n: i32, prices: Vec<Vec<i32>>) -> i64 {
        let m = m as usize;
        let n = n as usize;
        let mut dp = vec![vec![0; n + 1]; m + 1];
        
        for price in &prices {
            let h = price[0] as usize;
            let w = price[1] as usize;
            let p = price[2] as usize;
            dp[h][w] = dp[h][w].max(p);
        }
        
        for i in 0..=m {
            for j in 0..=n {
                for k in 0..j {
                    dp[i][j] = dp[i][j].max(dp[i][k] + dp[i][j - k]);
                }
                for k in 0..i {
                    dp[i][j] = dp[i][j].max(dp[k][j] + dp[i - k][j]);
                }
            }
        }
        
        dp[m][n] as i64

    }
}
