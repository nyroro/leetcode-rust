
use std::collections::HashMap;

impl Solution {
    pub fn wonderful_substrings(word: String) -> i64 {
        let mut result = 0;
        let word = word.as_bytes();
        for i in 0..word.len() {
            let mut count = HashMap::new();
            for j in i..word.len() {
                *count.entry(word[j]).or_insert(0) += 1;
                let mut odd_count = 0;
                for &val in count.values() {
                    if val % 2 != 0 {
                        odd_count += 1;
                    }
                }
                result += odd_count;
            }
        }
        result as i64

    }
}
