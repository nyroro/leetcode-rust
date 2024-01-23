


impl Solution {
    pub fn count_pyramids(grid: Vec<Vec<i32>>) -> i32 {
        let mut dp: Vec<Vec<i32>> = vec![vec![0; grid[0].len()]; grid.len()];
        let mut sums: Vec<Vec<i32>> = vec![vec![0; grid[0].len()]; grid.len()];

        for i in 0..grid.len() {
            sums[i][0] = grid[i][0];
            for j in 1..grid[0].len() {
                if grid[i][j] > 0 {
                    sums[i][j] = grid[i][j] + sums[i][j - 1];
                } else {
                    sums[i][j] = 0;
                }
            }
        }

        let mut ret = 0;
        for i in 1..grid.len() {
            for j in 1..grid[0].len() {
                if sums[i][j] >= 3 {
                    if sums[i - 1][j - 1] >= 1 {
                        let pre = dp[i - 1][j - 1];
                        if sums[i][j] >= 2 * (pre + 1) + 1 {
                            dp[i][j] = dp[i - 1][j - 1] + 1;
                        } else {
                            dp[i][j] = (sums[i][j] - 1) / 2;
                        }
                        ret += dp[i][j];
                    }
                }
            }
        }

        dp = vec![vec![0; grid[0].len()]; grid.len()];
        for i in (0..grid.len() - 1).rev() {
            for j in 1..grid[0].len() {
                if sums[i][j] >= 3 {
                    if sums[i + 1][j - 1] >= 1 {
                        let pre = dp[i + 1][j - 1];
                        if sums[i][j] >= 2 * (pre + 1) + 1 {
                            dp[i][j] = dp[i + 1][j - 1] + 1;
                        } else {
                            dp[i][j] = (sums[i][j] - 1) / 2;
                        }
                        ret += dp[i][j];
                    }
                }
            }
        }

        ret

    }
}
