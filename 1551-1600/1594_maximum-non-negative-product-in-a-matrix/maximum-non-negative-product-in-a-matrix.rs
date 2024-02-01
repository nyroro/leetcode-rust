


impl Solution {
    pub fn max_product_path(grid: Vec<Vec<i32>>) -> i32 {
        let m = grid.len();
        let n = grid[0].len();
        let mut dp: Vec<Vec<(i64, i64)>> = vec![(0, 0); n].iter().cloned().collect();

        for i in 0..n {
            if grid[0][i] >= 0 {
                dp[i].0 = dp.get(i.wrapping_sub(1)).unwrap_or(&(1, 1)).0 * grid[0][i] as i64;
                dp[i].1 = dp.get(i.wrapping_sub(1)).unwrap_or(&(1, 1)).1 * grid[0][i] as i64;
            } else {
                dp[i].0 = dp.get(i.wrapping_sub(1)).unwrap_or(&(1, 1)).1 * grid[0][i] as i64;
                dp[i].1 = dp.get(i.wrapping_sub(1)).unwrap_or(&(1, 1)).0 * grid[0][i] as i64;
            }
        }

        for i in 1..m {
            let mut next: Vec<(i64, i64)> = vec![(0, 0); n].iter().cloned().collect();
            if grid[i][0] >= 0 {
                next[0].0 = dp[0].0 * grid[i][0] as i64;
                next[0].1 = dp[0].1 * grid[i][0] as i64;
            } else {
                next[0].0 = dp[0].1 * grid[i][0] as i64;
                next[0].1 = dp[0].0 * grid[i][0] as i64;
            }
            for j in 1..n {
                if grid[i][j] >= 0 {
                    next[j].0 = std::cmp::max(next[j.wrapping_sub(1)].0, dp[j].0) * grid[i][j] as i64;
                    next[j].1 = std::cmp::min(next[j.wrapping_sub(1)].1, dp[j].1) * grid[i][j] as i64;
                } else {
                    next[j].0 = std::cmp::min(next[j.wrapping_sub(1)].1, dp[j].1) * grid[i][j] as i64;
                    next[j].1 = std::cmp::max(next[j.wrapping_sub(1)].0, dp[j].0) * grid[i][j] as i64;
                }
            }
            dp = next;
        }

        let modulo = 1_000_000_007;
        if dp[n - 1].0 >= 0 {
            (dp[n - 1].0 % modulo) as i32

        } else {
            -1

        }
    }
}
