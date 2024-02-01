
impl Solution {
    pub fn num_music_playlists(n: i32, goal: i32, k: i32) -> i32 {
        let modulo = 1_000_000_007;
        let mut dp = vec![vec![0; n as usize + 1]; goal as usize + 1];
        dp[0][0] = 1;
        
        for i in 1..=goal as usize {
            for j in 1..=n as usize {
                dp[i][j] = (dp[i-1][j-1] * (n-j+1) + dp[i-1][j] * (j.saturating_sub(k))) % modulo;
            }
        }
        
        dp[goal as usize][n as usize] as i32

    }
}
