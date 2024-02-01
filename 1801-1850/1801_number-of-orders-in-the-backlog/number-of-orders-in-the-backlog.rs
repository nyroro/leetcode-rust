
use std::collections::BinaryHeap;
use std::cmp::Reverse;

impl Solution {
    pub fn get_number_of_backlog_orders(orders: Vec<Vec<i32>>) -> i32 {
        let mut buy_orders: BinaryHeap<(i32, i32)> = BinaryHeap::new(); // max heap

        let mut sell_orders: BinaryHeap<(Reverse<i32>, i32)> = BinaryHeap::new(); // min heap
        
        for order in orders {
            let price = order[0];
            let mut amount = order[1]; // make amount mutable

            let order_type = order[2];
            
            if order_type == 0 {
                // handle buy orders

                while let Some(mut sell) = sell_orders.pop() {
                    if sell.0 .0 <= price {
                        let min_amount = sell.1.min(amount);
                        amount -= min_amount;
                        sell.1 -= min_amount;
                        if sell.1 > 0 {
                            sell_orders.push(sell);
                        }
                        if amount == 0 {
                            break;
                        }
                    } else {
                        sell_orders.push(sell);
                        break;
                    }
                }
                if amount > 0 {
                    buy_orders.push((price, amount));
                }
            } else {
                // handle sell orders

                while let Some(mut buy) = buy_orders.pop() {
                    if buy.0 >= price {
                        let min_amount = buy.1.min(amount);
                        amount -= min_amount;
                        buy.1 -= min_amount;
                        if buy.1 > 0 {
                            buy_orders.push(buy);
                        }
                        if amount == 0 {
                            break;
                        }
                    } else {
                        buy_orders.push(buy);
                        break;
                    }
                }
                if amount > 0 {
                    sell_orders.push((Reverse(price), amount));
                }
            }
        }
        
        let mut total_backlog = 0;
        let modulo = 1_000_000_007;
        
        for (price, amount) in buy_orders {
            total_backlog = (total_backlog + amount) % modulo;
        }
        
        for (Reverse(price), amount) in sell_orders {
            total_backlog = (total_backlog + amount) % modulo;
        }
        
        total_backlog as i32

    }
}
