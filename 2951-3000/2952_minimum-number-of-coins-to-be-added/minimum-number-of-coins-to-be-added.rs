
use std::collections::HashSet;

impl Solution {
    pub fn minimum_added_coins(coins: Vec<i32>, target: i32) -> i32 {
        let mut reachable = HashSet::new();
        reachable.insert(0);

        for coin in coins {
            let mut new_reachable = reachable.clone();
            for &value in reachable.iter() {
                if value + coin <= target {
                    new_reachable.insert(value + coin);
                }
            }
            reachable = new_reachable;
        }

        let mut count = 0;
        let mut max_reachable = 0;

        while max_reachable < target {
            count += 1;
            for i in max_reachable..=target {
                if !reachable.contains(&i) {
                    max_reachable = i - 1;
                    break;
                }
            }
            max_reachable += max_reachable + 1;
        }

        count

    }
}
