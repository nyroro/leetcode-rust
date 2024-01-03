
impl Solution {
    pub fn num_perms_di_sequence(s: String) -> i32 {
        const MOD: i32 = 1_000_000_007;
        let n = s.len();
        let mut dp = vec![vec![0; n + 1]; n + 1];
        dp[0][0] = 1;

        for i in 1..=n {
            if s.chars().nth(i - 1).unwrap() == 'D' {
                for j in (0..i).rev() {
                    dp[i][j] = (dp[i][j + 1] + dp[i - 1][j]) % MOD;
                }
            } else {
                for j in 1..=i {
                    dp[i][j] = (dp[i][j - 1] + dp[i - 1][j - 1]) % MOD;
                }
            }
        }

        let mut result = 0;
        for i in 0..=n {
            result = (result + dp[n][i]) % MOD;
        }

        result

    }
}
