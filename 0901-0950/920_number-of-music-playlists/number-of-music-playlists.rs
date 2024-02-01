
impl Solution {
    pub fn num_music_playlists(n: i32, goal: i32, k: i32) -> i32 {
        let modulo = 1_000_000_007;
        let n = n as usize;
        let goal = goal as usize;
        let k = k as usize;
        let mut dp = vec![vec![0; n + 1]; goal + 1];
        dp[0][0] = 1;
        
        for i in 1..=goal {
            for j in 1..=n {
                dp[i][j] = ((dp[i-1][j-1] as i64 * (n-j+1) as i64) % modulo as i64 + (dp[i-1][j] as i64 * (j.saturating_sub(k) as i64)) % modulo as i64) % modulo as i64;
            }
        }
        
        dp[goal][n] as i32

    }
}
