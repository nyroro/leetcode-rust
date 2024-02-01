
impl Solution {
    pub fn stone_game_vii(stones: Vec<i32>) -> i32 {
        let n = stones.len();
        let mut prefix_sum = vec![0; n + 1];
        for i in 0..n {
            prefix_sum[i + 1] = prefix_sum[i] + stones[i];
        }
        
        let mut dp = vec![vec![0; n]; n];
        return Self::dfs(0, n - 1, &mut dp, &prefix_sum);
    }
    
    fn dfs(left: usize, right: usize, dp: &mut Vec<Vec<i32>>, prefix_sum: &Vec<i32>) -> i32 {
        if left == right {
            return 0;
        }
        
        if dp[left][right] != 0 {
            return dp[left][right];
        }
        
        let score_left = prefix_sum[right + 1] - prefix_sum[left + 1] + Self::dfs(left + 1, right, dp, prefix_sum);
        let score_right = prefix_sum[right] - prefix_sum[left] + Self::dfs(left, right - 1, dp, prefix_sum);
        
        dp[left][right] = score_left.max(score_right);
        return dp[left][right];
    }
}
