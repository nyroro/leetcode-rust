
impl Solution {
    pub fn minimum_lines(stock_prices: Vec<Vec<i32>>) -> i32 {
        if stock_prices.len() == 1 {
            return 0;
        }
        
        let mut stock_prices = stock_prices;
        stock_prices.sort();
        
        let mut ret = 1;
        let mut a = stock_prices[0];
        let mut b = stock_prices[1];
        
        for i in 2..stock_prices.len() {
            let c = &stock_prices[i];
            
            let x1 = a[0] - b[0];
            let x2 = c[0] - b[0];
            let y1 = a[1] - b[1];
            let y2 = c[1] - b[1];
            
            if x1 * y2 != x2 * y1 {
                ret += 1;
            }
            
            a = b;
            b = c.clone();
        }
        
        ret

    }
}
