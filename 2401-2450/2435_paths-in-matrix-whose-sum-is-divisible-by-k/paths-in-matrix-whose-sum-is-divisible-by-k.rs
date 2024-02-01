
use std::collections::HashSet;

impl Solution {
    pub fn number_of_paths(grid: Vec<Vec<i32>>, k: i32) -> i32 {
        let m = grid.len();
        let n = grid[0].len();
        let mut dp = vec![vec![vec![0; k as usize]; n]; m];
        dp[0][0][grid[0][0] as usize % k as usize] = 1;

        for i in 0..m {
            for j in 0..n {
                if i > 0 {
                    for r in 0..k as usize {
                        let new_remainder = (r as i32 + grid[i][j]) % k;
                        dp[i][j][new_remainder as usize] += dp[i - 1][j][r];
                        dp[i][j][new_remainder as usize] %= 1_000_000_007;
                    }
                }
                if j > 0 {
                    for r in 0..k as usize {
                        let new_remainder = (r as i32 + grid[i][j]) % k;
                        dp[i][j][new_remainder as usize] += dp[i][j - 1][r];
                        dp[i][j][new_remainder as usize] %= 1_000_000_007;
                    }
                }
            }
        }
        dp[m - 1][n - 1][0]
    }
}
