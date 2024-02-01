
impl Solution {
    pub fn count_palindromic_subsequences(s: String) -> i32 {
        let n = s.len();
        let mut dp = vec![vec![vec![0; 4]; n]; n];
        let s_bytes = s.as_bytes();
        let mod_num = 1000000007;

        for i in 0..n {
            dp[i][i][((s_bytes[i] - b'a') as usize)] = 1;
        }

        for i in 1..n {
            for j in 0..n - i {
                let end = j + i;
                if s_bytes[j] == s_bytes[end] {
                    dp[j][end][(s_bytes[j] - b'a') as usize] = 2;
                }
                for k in 0..4 {
                    if s_bytes[j] == s_bytes[end] {
                        dp[j][end][(s_bytes[j] - b'a') as usize] += dp[j + 1][end - 1][k];
                        dp[j][end][(s_bytes[j] - b'a') as usize] %= mod_num;
                    }
                    dp[j][end][k] += (dp[j][end - 1][k] + dp[j + 1][end][k]) % mod_num;
                    dp[j][end][k] %= mod_num;
                    dp[j][end][k] += mod_num - dp[j + 1][end - 1][k];
                    dp[j][end][k] %= mod_num;
                }
            }
        }

        let mut result = 0;
        for k in 0..4 {
            result += dp[0][n - 1][k];
            result %= mod_num;
        }

        result as i32

    }
}
