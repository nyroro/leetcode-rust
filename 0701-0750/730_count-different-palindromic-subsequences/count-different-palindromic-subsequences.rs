
impl Solution {
    pub fn count_palindromic_subsequences(s: String) -> i32 {
        let n = s.len();
        let s_bytes = s.as_bytes();
        let mod_num = 1000000007;
        let mut dp = vec![vec![0; n]; n];
        for i in 0..n {
            dp[i][i] = 1;
        }

        for len in 2..=n {
            for i in 0..=n - len {
                let j = i + len - 1;
                if s_bytes[i] == s_bytes[j] {
                    let mut left = i + 1;
                    let mut right = j - 1;
                    while left <= right && s_bytes[left] != s_bytes[i] {
                        left += 1;
                    }
                    while left <= right && s_bytes[right] != s_bytes[i] {
                        right -= 1;
                    }
                    if left > right {
                        dp[i][j] = dp[i + 1][j - 1] * 2 + 2;
                    } else if left == right {
                        dp[i][j] = dp[i + 1][j - 1] * 2 + 1;
                    } else {
                        dp[i][j] = dp[i + 1][j - 1] * 2 - dp[left + 1][right - 1];
                    }
                } else {
                    dp[i][j] = dp[i][j - 1] + dp[i + 1][j] - dp[i + 1][j - 1];
                }
                dp[i][j] = if dp[i][j] < 0 {
                    dp[i][j] + mod_num

                } else {
                    dp[i][j] % mod_num

                };
            }
        }

        dp[0][n - 1] as i32

    }
}
