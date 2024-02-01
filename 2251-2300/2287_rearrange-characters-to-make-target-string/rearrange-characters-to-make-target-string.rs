
use std::collections::HashMap;

impl Solution {
    pub fn rearrange_characters(s: String, target: String) -> i32 {
        let mut s_count = HashMap::new();
        for c in s.chars() {
            *s_count.entry(c).or_insert(0) += 1;
        }
        
        let mut max_copies = i32::max_value();
        for c in target.chars() {
            let count = s_count.get(&c).unwrap_or(&0);
            if *count == 0 {
                return 0;
            }
            max_copies = max_copies.min(*count as i32);
        }
        
        max_copies

    }
}
