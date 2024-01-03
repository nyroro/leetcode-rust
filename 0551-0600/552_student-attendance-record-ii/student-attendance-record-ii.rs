
impl Solution {
    pub fn check_record(n: i32) -> i32 {
        let modulo = 1000000007;
        let n = n as usize;
        let mut dp = vec![vec![vec![0; 3]; 2]; n + 1];
        dp[0][0][0] = 1;

        for i in 1..=n {
            // 以P结尾

            for j in 0..=1 {
                for k in 0..=2 {
                    dp[i][j][0] = (dp[i][j][0] + dp[i - 1][j][k]) % modulo;
                }
            }
            // 以A结尾

            for k in 0..=2 {
                dp[i][1][0] = (dp[i][1][0] + dp[i - 1][0][k]) % modulo;
            }
            // 以L结尾

            for j in 0..=1 {
                for k in 1..=2 {
                    dp[i][j][k] = (dp[i][j][k] + dp[i - 1][j][k - 1]) % modulo;
                }
            }
        }

        let mut res = 0;
        for j in 0..=1 {
            for k in 0..=2 {
                res = (res + dp[n][j][k]) % modulo;
            }
        }
        res

    }
}
