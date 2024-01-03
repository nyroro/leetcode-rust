
impl Solution {
    pub fn knight_dialer(n: i32) -> i32 {
        let modulo = 1000000007;
        let mut dp = vec![vec![0; 10]; n as usize];
        
        // 初始化第 0 步的值

        for j in 0..10 {
            dp[0][j] = 1;
        }
        
        // 计算每一步的值

        for i in 1..n as usize {
            dp[i][0] = (dp[i-1][4] + dp[i-1][6]) % modulo;
            dp[i][1] = (dp[i-1][6] + dp[i-1][8]) % modulo;
            dp[i][2] = (dp[i-1][7] + dp[i-1][9]) % modulo;
            dp[i][3] = (dp[i-1][4] + dp[i-1][8]) % modulo;
            dp[i][4] = ((dp[i-1][0] + dp[i-1][3]) % modulo + dp[i-1][9]) % modulo;
            dp[i][6] = ((dp[i-1][0] + dp[i-1][1]) % modulo + dp[i-1][7]) % modulo;
            dp[i][7] = (dp[i-1][2] + dp[i-1][6]) % modulo;
            dp[i][8] = (dp[i-1][1] + dp[i-1][3]) % modulo;
            dp[i][9] = (dp[i-1][2] + dp[i-1][4]) % modulo;
        }
        
        // 计算总数量

        let mut total = 0;
        for j in 0..10 {
            total = (total + dp[n as usize - 1][j]) % modulo;
        }
        
        total

    }
}
