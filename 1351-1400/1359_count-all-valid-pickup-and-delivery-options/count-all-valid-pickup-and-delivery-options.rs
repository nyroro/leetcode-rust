
impl Solution {
    pub fn count_orders(n: i32) -> i32 {
        let modulo = 1000000007;
        let mut dp = vec![0; (n+1) as usize];
        dp[1] = 1;
        
        for i in 2..=n as usize {
            dp[i] = (dp[i-1] as i64 * (2*i-1) as i64 * (2*i) as i64 / 2) as i32 % modulo;
        }
        
        dp[n as usize]
    }
}
