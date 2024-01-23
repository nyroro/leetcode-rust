
impl Solution {
    pub fn get_descent_periods(prices: Vec<i32>) -> i64 {
        let mut pre = vec![-1; prices.len()];
        let mut dp = vec![0; prices.len()];
        
        for i in 0..prices.len() {
            dp[i] = 1;
            pre[i] = i as i32;
            if i > 0 {
                if prices[i] == prices[i-1]-1 {
                    pre[i] = pre[i-1];
                    dp[i] = (i as i32 - pre[i] + 1).max(dp[i]);
                }
            }
        }
        
        dp.iter().map(|&x| x as i64).sum()
    }
}
