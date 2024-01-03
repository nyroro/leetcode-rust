
use std::collections::HashMap;

impl Solution {
    pub fn wonderful_substrings(word: String) -> i64 {
        let mut result = 0;
        let word = word.as_bytes();
        let mut count = HashMap::new();
        count.insert(0, 1);
        let mut mask = 0;
        for i in 0..word.len() {
            let c = (word[i] - b'a') as usize;
            mask ^= 1 << c;
            result += *count.entry(mask).or_insert(0);
            for j in 0..10 {
                let new_mask = mask ^ (1 << j);
                result += *count.get(&new_mask).unwrap_or(&0);
            }
            *count.entry(mask).or_insert(0) += 1;
        }
        result

    }
}
