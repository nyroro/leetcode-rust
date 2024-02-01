


impl Solution {
    pub fn max_product_path(grid: Vec<Vec<i32>>) -> i32 {
        let m = grid.len();
        let n = grid[0].len();
        let mut dp: Vec<Vec<(i64, i64)>> = vec![vec![(0, 0); n]; m];

        dp[0][0] = (grid[0][0] as i64, grid[0][0] as i64);

        for i in 1..m {
            dp[i][0].0 = dp[i - 1][0].0 * grid[i][0] as i64;
            dp[i][0].1 = dp[i - 1][0].1 * grid[i][0] as i64;
        }

        for j in 1..n {
            dp[0][j].0 = dp[0][j - 1].0 * grid[0][j] as i64;
            dp[0][j].1 = dp[0][j - 1].1 * grid[0][j] as i64;
        }

        for i in 1..m {
            for j in 1..n {
                if grid[i][j] >= 0 {
                    dp[i][j].0 = std::cmp::max(dp[i - 1][j].0, dp[i][j - 1].0) * grid[i][j] as i64;
                    dp[i][j].1 = std::cmp::min(dp[i - 1][j].1, dp[i][j - 1].1) * grid[i][j] as i64;
                } else {
                    dp[i][j].0 = std::cmp::min(dp[i - 1][j].1, dp[i][j - 1].1) * grid[i][j] as i64;
                    dp[i][j].1 = std::cmp::max(dp[i - 1][j].0, dp[i][j - 1].0) * grid[i][j] as i64;
                }
            }
        }

        let modulo = 1_000_000_007;
        if dp[m - 1][n - 1].0 >= 0 {
            (dp[m - 1][n - 1].0 % modulo) as i32

        } else {
            -1

        }
    }
}
