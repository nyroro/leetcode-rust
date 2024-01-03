
impl Solution {
    pub fn k_inverse_pairs(n: i32, k: i32) -> i32 {
        const MOD: i32 = 1_000_000_007;
        let mut dp = vec![vec![0; k as usize + 1]; n as usize + 1];
        dp[0][0] = 1;

        for i in 1..=n as usize {
            dp[i][0] = 1;
            for j in 1..=k as usize {
                let mut sum = (dp[i-1][j] + MOD - (if j >= i { dp[i-1][j-i] } else { 0 })) % MOD;
                sum = (sum + dp[i][j-1]) % MOD;
                dp[i][j] = sum;
            }
        }

        dp[n as usize][k as usize]
    }
}
