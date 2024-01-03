
use std::collections::HashMap;

impl Solution {
    pub fn longest_palindrome(s: String) -> i32 {
        let mut count_map: HashMap<char, i32> = HashMap::new();
        let mut length = 0;
        let mut has_odd = false;
        
        for c in s.chars() {
            *count_map.entry(c).or_insert(0) += 1;
        }
        
        for (_, count) in count_map {
            if count % 2 == 0 {
                length += count;
            } else {
                length += count - 1;
                has_odd = true;
            }
        }
        
        if has_odd {
            length += 1;
        }
        
        length

    }
}
