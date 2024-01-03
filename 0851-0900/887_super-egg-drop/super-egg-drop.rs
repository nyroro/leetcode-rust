
impl Solution {
    pub fn super_egg_drop(k: i32, n: i32) -> i32 {
        let k = k as usize;
        let n = n as usize;
        let mut dp = vec![vec![0; n + 1]; k + 1];

        for i in 1..=n {
            dp[1][i] = i;
        }

        for i in 2..=k {
            let mut x = 1;
            for j in 1..=n {
                while x < j && std::cmp::max(dp[i - 1][x - 1], dp[i][j - x]) > std::cmp::max(dp[i - 1][x], dp[i][j - x - 1]) {
                    x += 1;
                }
                dp[i][j] = 1 + std::cmp::max(dp[i - 1][x - 1], dp[i][j - x]);
            }
        }

        dp[k][n] as i32

    }
}
