
use std::collections::HashSet;



impl Solution {
    pub fn partition_string(s: String) -> i32 {
        let mut unique_chars = HashSet::new();
        let mut substrings = 0;

        for c in s.chars() {
            if unique_chars.contains(&c) {
                substrings += 1;
                unique_chars.clear();
            }
            unique_chars.insert(c);
        }

        substrings + 1

    }
}
