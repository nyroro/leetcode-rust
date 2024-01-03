
use std::collections::HashMap;

impl Solution {
    pub fn find_the_difference(s: String, t: String) -> char {
        let mut char_count: HashMap<char, i32> = HashMap::new();
        
        // Count characters in s

        for c in s.chars() {
            let count = char_count.entry(c).or_insert(0);
            *count += 1;
        }
        
        // Find the added character in t

        for c in t.chars() {
            if !char_count.contains_key(&c) {
                return c;
            }
            
            let count = char_count.get_mut(&c).unwrap();
            *count -= 1;
            
            if *count < 0 {
                return c;
            }
        }
        
        // Return the last character in t

        t.chars().last().unwrap()
    }
}
