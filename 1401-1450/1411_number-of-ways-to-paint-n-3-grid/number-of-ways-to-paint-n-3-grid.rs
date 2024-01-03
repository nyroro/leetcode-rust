
impl Solution {
    pub fn num_of_ways(n: i32) -> i32 {
        let k_mod: i64 = 1_000_000_007;
        let mut dp = vec![(6, 6); (n + 1) as usize];
        
        for i in 2..=n as usize {
            let prev_dp = dp[i - 1];
            let dp0 = (prev_dp.0 * 3 + prev_dp.1 * 2) % k_mod;
            let dp1 = (prev_dp.0 * 2 + prev_dp.1 * 2) % k_mod;
            dp[i] = (dp0, dp1);
        }
        
        let (dp0, dp1) = dp[n as usize];
        ((dp0 + dp1) % k_mod) as i32

    }
}
