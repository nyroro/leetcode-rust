
impl Solution {
    pub fn shortest_path(grid: Vec<Vec<i32>>, k: i32) -> i32 {
        let m = grid.len();
        let n = grid[0].len();
        let mut dp = vec![vec![vec![std::i32::MAX; (k + 1) as usize]; n]; m];
        dp[0][0][0] = 0;

        for obstacles in 0..=k {
            for i in 0..m {
                for j in 0..n {
                    if i > 0 {
                        dp[i][j][obstacles as usize] = dp[i][j][obstacles as usize].min(dp[i - 1][j][obstacles as usize] + grid[i][j]);
                    }
                    if j > 0 {
                        dp[i][j][obstacles as usize] = dp[i][j][obstacles as usize].min(dp[i][j - 1][obstacles as usize] + grid[i][j]);
                    }
                    if i < m - 1 {
                        dp[i][j][obstacles as usize] = dp[i][j][obstacles as usize].min(dp[i + 1][j][obstacles as usize] + grid[i][j]);
                    }
                    if j < n - 1 {
                        dp[i][j][obstacles as usize] = dp[i][j][obstacles as usize].min(dp[i][j + 1][obstacles as usize] + grid[i][j]);
                    }
                    if grid[i][j] == 1 && obstacles < k {
                        dp[i][j][(obstacles + 1) as usize] = dp[i][j][(obstacles + 1) as usize].min(dp[i][j][obstacles as usize] + 1);
                    }
                }
            }
        }

        if dp[m - 1][n - 1][k as usize] == std::i32::MAX {
            -1

        } else {
            dp[m - 1][n - 1][k as usize]
        }
    }
}
