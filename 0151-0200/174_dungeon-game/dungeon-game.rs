
impl Solution {
    pub fn calculate_minimum_hp(dungeon: Vec<Vec<i32>>) -> i32 {
        let m = dungeon.len();
        let n = dungeon[0].len();
        let mut dp = vec![vec![0; n]; m];

        for i in (0..m).rev() {
            for j in (0..n).rev() {
                if i == m - 1 && j == n - 1 {
                    dp[i][j] = dungeon[i][j].max(0);
                } else if i == m - 1 {
                    dp[i][j] = (dp[i][j + 1] - dungeon[i][j]).max(0);
                } else if j == n - 1 {
                    dp[i][j] = (dp[i + 1][j] - dungeon[i][j]).max(0);
                } else {
                    dp[i][j] = (dp[i + 1][j].min(dp[i][j + 1]) - dungeon[i][j]).max(0);
                }
            }
        }

        dp[0][0] + 1

    }
}
