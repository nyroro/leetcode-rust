
impl Solution {
    pub fn stone_game_viii(stones: Vec<i32>) -> i32 {
        let n = stones.len();
        let mut prefix_sum = vec![0; n];
        prefix_sum[0] = stones[0];
        
        for i in 1..n {
            prefix_sum[i] = prefix_sum[i - 1] + stones[i];
        }
        
        let mut dp = vec![0; n];
        dp[n - 2] = prefix_sum[n - 1];
        
        for i in (1..n - 1).rev() {
            dp[i] = std::cmp::max(dp[i + 1], prefix_sum[i] - dp[i + 1]);
        }
        
        dp[1]
    }
}
