
impl Solution {
    pub fn num_of_arrays(n: i32, m: i32, k: i32) -> i32 {
        const MOD: i64 = 1000000007;
        let mut dp = vec![vec![vec![0; (k + 1) as usize]; (m + 1) as usize]; (n + 1) as usize];
        for j in 1..=m as usize {
            dp[1][j][1] = 1;
        }
        for i in 2..=n as usize {
            for j in 1..=m as usize {
                for p in 1..=k as usize {
                    for x in 1..j {
                        dp[i][j][p] = (dp[i][j][p] + dp[i - 1][x][p - 1]) % MOD;
                    }
                    dp[i][j][p] = (dp[i][j][p] + dp[i - 1][j][p] * j as i64) % MOD;
                }
            }
        }
        let mut result = 0;
        for j in 1..=m as usize {
            result = (result + dp[n as usize][j][k as usize]) % MOD;
        }
        result as i32

    }
}
