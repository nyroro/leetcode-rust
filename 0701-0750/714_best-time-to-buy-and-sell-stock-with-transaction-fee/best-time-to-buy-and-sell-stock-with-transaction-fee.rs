
impl Solution {
    pub fn max_profit(prices: Vec<i32>, fee: i32) -> i32 {
        let mut cash = 0;
        let mut hold = -prices[0];

        for i in 1..prices.len() {
            let price = prices[i];
            let prev_cash = cash;
            cash = cash.max(hold + price - fee);
            hold = hold.max(prev_cash - price);
        }

        cash

    }
}
