
impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let n = prices.len();
        if n < 2 {
            return 0;
        }
        
        let mut buy = vec![0; n];
        let mut sell = vec![0; n];
        let mut cooldown = vec![0; n];
        
        buy[0] = -prices[0];
        
        for i in 1..n {
            buy[i] = buy[i-1].max(cooldown[i-1] - prices[i]);
            sell[i] = sell[i-1].max(buy[i-1] + prices[i]);
            cooldown[i] = sell[i-1];
        }
        
        sell[n-1]
    }
}
