
impl Solution {
    pub fn max_profit(k: i32, prices: Vec<i32>) -> i32 {
        let n = prices.len();
        if k >= n as i32 / 2 {
            // 可以进行无限次交易

            let mut max_profit = 0;
            for i in 1..n {
                if prices[i] > prices[i-1] {
                    max_profit += prices[i] - prices[i-1];
                }
            }
            return max_profit;
        }
        
        let k = k as usize;
        let mut buy = vec![-prices[0]; k+1];
        let mut sell = vec![0; k+1];
        
        for i in 1..n {
            for j in 1..=k {
                buy[j] = buy[j].max(sell[j-1] - prices[i]);
                sell[j] = sell[j].max(buy[j] + prices[i]);
            }
        }
        
        sell[k]
    }
}
