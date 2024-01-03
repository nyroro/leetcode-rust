
impl Solution {
    pub fn is_scramble(s1: String, s2: String) -> bool {
        let n = s1.len();
        if n != s2.len() {
            return false;
        }

        let s1_chars: Vec<char> = s1.chars().collect();
        let s2_chars: Vec<char> = s2.chars().collect();

        let mut dp = vec![vec![vec![false; n]; n]; n + 1];

        // 初始化长度为1的情况

        for i in 0..n {
            for j in 0..n {
                if s1_chars[i] == s2_chars[j] {
                    dp[1][i][j] = true;
                }
            }
        }

        // 枚举长度

        for len in 2..=n {
            // 枚举s1的起点

            for i in 0..=n - len {
                // 枚举s2的起点

                for j in 0..=n - len {
                    // 枚举划分位置

                    for k in 1..len {
                        // 不交换的情况

                        if dp[k][i][j] && dp[len - k][i + k][j + k] {
                            dp[len][i][j] = true;
                            break;
                        }
                        // 交换的情况

                        if dp[k][i][j + len - k] && dp[len - k][i + k][j] {
                            dp[len][i][j] = true;
                            break;
                        }
                    }
                }
            }
        }

        dp[n][0][0]
    }
}
