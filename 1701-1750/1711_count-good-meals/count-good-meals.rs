
use std::collections::HashMap;

impl Solution {
    pub fn count_pairs(deliciousness: Vec<i32>) -> i32 {
        let mut count = 0;
        let mut map = HashMap::new();
        let modulo = 1000000007;

        for &d in deliciousness.iter() {
            for i in 0..22 {
                let target = 1 << i;
                let complement = target - d;
                if let Some(&c) = map.get(&complement) {
                    count = (count + c) % modulo;
                }
            }
            *map.entry(d).or_insert(0) += 1;
        }
        count

    }
}
