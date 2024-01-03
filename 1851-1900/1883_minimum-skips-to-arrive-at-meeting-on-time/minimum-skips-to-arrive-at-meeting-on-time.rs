
impl Solution {
    pub fn min_skips(dist: Vec<i32>, speed: i32, hours_before: i32) -> i32 {
        let n = dist.len();
        let mut dp = vec![vec![i32::MAX / 2; n + 1]; n + 1];
        dp[0][0] = 0;

        for i in 0..=n {
            for j in 0..=i {
                if j != i {
                    dp[i][j] = dp[i][j].min(((dp[i - 1][j] + dist[i - 1] - 1) / speed + 1) * speed);
                }
                if j != 0 {
                    dp[i][j] = dp[i][j].min(dp[i - 1][j - 1] + dist[i - 1]);
                }
            }
        }

        for j in 0..=n {
            if dp[n][j] as i64 <= hours_before as i64 * speed as i64 {
                return j as i32;
            }
        }

        -1

    }
}
