
struct StockSpanner {
    prices: Vec<i32>,
    spans: Vec<i32>,
}

impl StockSpanner {
    fn new() -> Self {
        StockSpanner {
            prices: Vec::new(),
            spans: Vec::new(),
        }
    }
    
    fn next(&mut self, price: i32) -> i32 {
        let mut span = 1;
        while let Some(&prev_price) = self.prices.last() {
            if prev_price <= price {
                span += self.spans.pop().unwrap();
                self.prices.pop();
            } else {
                break;
            }
        }
        self.prices.push(price);
        self.spans.push(span);
        span

    }
}

fn main() {
    let mut stock_spanner = StockSpanner::new();
    println!("{}", stock_spanner.next(100)); // return 1

    println!("{}", stock_spanner.next(80));  // return 1

    println!("{}", stock_spanner.next(60));  // return 1

    println!("{}", stock_spanner.next(70));  // return 2

    println!("{}", stock_spanner.next(60));  // return 1

    println!("{}", stock_spanner.next(75));  // return 4

    println!("{}", stock_spanner.next(85));  // return 6

}
