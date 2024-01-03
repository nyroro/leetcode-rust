
impl Solution {
    pub fn mincost_tickets(days: Vec<i32>, costs: Vec<i32>) -> i32 {
        let mut dp = vec![0; 366];
        let mut travel_days = vec![false; 366];
        for day in days {
            travel_days[day as usize] = true;
        }
        
        for i in 1..366 {
            if !travel_days[i] {
                dp[i] = dp[i - 1];
                continue;
            }
            
            let mut min_cost = dp[i - 1] + costs[0];
            min_cost = min_cost.min(dp[i.saturating_sub(7)] + costs[1]);
            min_cost = min_cost.min(dp[i.saturating_sub(30)] + costs[2]);
            
            dp[i] = min_cost;
        }
        
        dp[365]
    }
}
