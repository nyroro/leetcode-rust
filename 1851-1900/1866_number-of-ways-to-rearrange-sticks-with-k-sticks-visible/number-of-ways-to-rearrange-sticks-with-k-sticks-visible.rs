
impl Solution {
    pub fn rearrange_sticks(n: i32, k: i32) -> i32 {
        let mut dp = vec![vec![0; (k + 1) as usize]; (n + 1) as usize];
        let modulo = 1_000_000_007;
        
        dp[1][1] = 1;
        
        for i in 2..=n as usize {
            for j in 1..=k as usize {
                dp[i][j] = (dp[i - 1][j - 1] + ((i - 1) as i64 * dp[i - 1][j] as i64) % modulo) as i32;
                dp[i][j] %= modulo;
            }
        }
        
        dp[n as usize][k as usize]
    }
}
