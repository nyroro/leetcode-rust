
struct Chocolate {
    price: i32,
    index: usize,
}

impl Solution {
    pub fn buy_choco(prices: Vec<i32>, money: i32) -> i32 {
        let mut chocolates: Vec<Chocolate> = prices.iter().enumerate().map(|(i, &p)| Chocolate { price: p, index: i }).collect();
        chocolates.sort_by_key(|c| c.price);

        let mut min_price = std::i32::MAX;

        for i in 0..chocolates.len() {
            for j in (i + 1)..chocolates.len() {
                let total_price = chocolates[i].price + chocolates[j].price;
                if total_price <= money {
                    min_price = min_price.min(total_price);
                }
            }
        }

        if min_price == std::i32::MAX {
            money

        } else {
            money - min_price

        }
    }
}
