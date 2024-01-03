
use std::collections::HashMap;

impl Solution {
    pub fn num_splits(s: String) -> i32 {
        let mut left_map: HashMap<char, i32> = HashMap::new();
        let mut right_map: HashMap<char, i32> = HashMap::new();
        let mut good_splits = 0;

        for c in s.chars() {
            *right_map.entry(c).or_insert(0) += 1;
        }

        for c in s.chars() {
            *left_map.entry(c).or_insert(0) += 1;
            *right_map.entry(c).or_insert(0) -= 1;

            if right_map[&c] == 0 {
                right_map.remove(&c);
            }

            if left_map.len() == right_map.len() {
                good_splits += 1;
            }
        }

        good_splits

    }
}
