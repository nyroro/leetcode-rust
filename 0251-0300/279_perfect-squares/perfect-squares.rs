
impl Solution {
    pub fn num_squares(n: i32) -> i32 {
        let n = n as usize;
        let mut dp = vec![n + 1; n + 1];
        dp[0] = 0;
        
        for i in 1..=n {
            for j in 1..=(i as f64).sqrt() as usize {
                dp[i] = dp[i].min(dp[i - j * j] + 1);
            }
        }
        
        dp[n] as i32

    }
}
