
use std::collections::HashMap;

impl Solution {
    pub fn min_cost(basket1: Vec<i32>, basket2: Vec<i32>) -> i64 {
        let mut freq: HashMap<i32, i32> = HashMap::new();
        let mut ans: i64 = 0;

        for b in &basket1 {
            *freq.entry(*b).or_insert(0) += 1;
        }

        for b in &basket2 {
            *freq.entry(*b).or_insert(0) -= 1;
        }

        let mut keys: Vec<i32> = freq.keys().cloned().collect();
        keys.sort();

        let mut swap_from_b1_to_b2: Vec<i32> = Vec::new();
        let mut swap_from_b2_to_b1: Vec<i32> = Vec::new();

        for &key in &keys {
            let count = freq[&key];
            if count % 2 != 0 {
                return -1;
            } else if count > 0 {
                let temp = count / 2;
                for _ in 0..temp {
                    swap_from_b1_to_b2.push(key);
                }
            } else if count < 0 {
                let temp = (-count) / 2;
                for _ in 0..temp {
                    swap_from_b2_to_b1.push(key);
                }
            }
        }

        swap_from_b2_to_b1.reverse();

        for i in 0..swap_from_b1_to_b2.len() {
            let indirect_swap_cost = 2 * keys[0];
            let direct_swap_cost = std::cmp::min(swap_from_b1_to_b2[i], swap_from_b2_to_b1[i]);
            ans += std::cmp::min(indirect_swap_cost, direct_swap_cost) as i64;
        }

        ans

    }
}
