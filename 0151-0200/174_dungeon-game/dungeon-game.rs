
impl Solution {
    pub fn calculate_minimum_hp(dungeon: Vec<Vec<i32>>) -> i32 {
        let m = dungeon.len() as i32;
        let n = dungeon[0].len() as i32;
        let mut dp = vec![vec![i32::MAX; n as usize + 1]; m as usize + 1];
        dp[m as usize][n as usize - 1] = 1;
        dp[m as usize - 1][n as usize] = 1;

        for i in (0..m).rev() {
            for j in (0..n).rev() {
                let need = (dp[i as usize + 1][j as usize].min(dp[i as usize][j as usize + 1]) - dungeon[i as usize][j as usize]).max(1);
                dp[i as usize][j as usize] = need;
            }
        }

        dp[0][0]
    }
}
