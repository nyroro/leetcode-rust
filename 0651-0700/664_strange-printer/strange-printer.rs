
impl Solution {
    pub fn strange_printer(s: String) -> i32 {
        // 将字符串转换为字符数组

        let chars: Vec<char> = s.chars().collect();
        // 获取字符串的长度

        let n = chars.len();
        // 创建一个二维数组dp，用于记录打印字符的最小次数

        let mut dp = vec![vec![0; n]; n];
        
        // 动态规划求解最小次数

        for len in (0..n).rev() {
            for j in len..n {
                if len == j {
                    // 当len和j相等时，表示只有一个字符，打印次数为1

                    dp[len][j] = 1;
                } else {
                    // 初始化最小次数为打印len到j-1的字符的最小次数加1

                    dp[len][j] = dp[len][j - 1] + 1;
                    // 遍历len到j-1的字符，找到一个字符k，使得chars[k] == chars[j]
                    for k in len..j {
                        if chars[k] == chars[j] {
                            // 更新最小次数为打印len到k的字符的最小次数加上打印k+1到j-1的字符的最小次数

                            dp[len][j] = dp[len][j].min(dp[len][k] + dp[k + 1][j - 1]);
                        }
                    }
                }
            }
        }
        
        // 返回打印字符串s的最小次数

        dp[0][n - 1]
    }
}
