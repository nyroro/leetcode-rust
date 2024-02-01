
impl Solution {
    pub fn paint_walls(cost: Vec<i32>, time: Vec<i32>) -> i32 {
        let n = cost.len();
        let mut dp = vec![vec![1e9 as i32; n + 1]; 2];
        dp[n & 1][n] = 0;

        for i in (0..n).rev() {
            dp[i & 1][n] = 0;
            for walls in (0..=n).rev() {
                let paint = cost[i] + dp[(i + 1) & 1][std::cmp::min(n, walls + 1 + time[i] as usize)];
                dp[i & 1][walls] = std::cmp::min(paint, dp[(i + 1) & 1][walls]);
            }
        }

        dp[0][0]
    }
}
