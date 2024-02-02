
use std::collections::HashMap;



impl Solution {
    pub fn max_partitions_after_operations(s: String, k: i32) -> i32 {
        let mut memo: HashMap<(usize, u32, bool), i32> = HashMap::new();

        fn dp(index: usize, current_set: u32, can_change: bool, s: &String, k: i32, memo: &mut HashMap<(usize, u32, bool), i32>) -> i32 {
            if let Some(&result) = memo.get(&(index, current_set, can_change)) {
                return result;
            }

            if index == s.len() {
                return 0;
            }

            let character_index = (s.as_bytes()[index] - b'a') as u32;
            let current_set_updated = current_set | (1 << character_index);
            let mut res: i32;

            let distinct_count = current_set_updated.count_ones() as i32;

            if distinct_count > k {
                res = 1 + dp(index + 1, 1 << character_index, can_change, s, k, memo);
            } else {
                res = dp(index + 1, current_set_updated, can_change, s, k, memo);
            }

            if can_change {
                for new_char_index in 0..26 {
                    let new_set = current_set | (1 << new_char_index);
                    let new_distinct_count = new_set.count_ones() as i32;

                    if new_distinct_count > k {
                        res = res.max(1 + dp(index + 1, 1 << new_char_index, false, s, k, memo));
                    } else {
                        res = res.max(dp(index + 1, new_set, false, s, k, memo));
                    }
                }
            }

            memo.insert((index, current_set, can_change), res);
            res

        }

        dp(0, 0, true, &s, k, &mut memo) + 1

    }
}
