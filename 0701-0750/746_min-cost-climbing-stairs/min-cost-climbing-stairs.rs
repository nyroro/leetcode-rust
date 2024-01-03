
impl Solution {
    pub fn min_cost_climbing_stairs(cost: Vec<i32>) -> i32 {
        let n = cost.len();
        let mut dp = vec![0; n];
        dp[0] = cost[0];
        dp[1] = cost[1];
        
        for i in 2..n {
            dp[i] = cost[i] + dp[i-1].min(dp[i-2]);
        }
        
        dp[n-1].min(dp[n-2])
    }
}
