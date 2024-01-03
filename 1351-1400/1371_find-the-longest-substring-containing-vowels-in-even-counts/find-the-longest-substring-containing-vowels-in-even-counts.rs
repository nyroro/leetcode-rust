
use std::collections::HashMap;

impl Solution {
    pub fn find_the_longest_substring(s: String) -> i32 {
        let vowels = ['a', 'e', 'i', 'o', 'u'];
        let mut counts: HashMap<u32, i32> = HashMap::new();
        counts.insert(0, -1);
        let mut mask = 0;
        let mut max_length = 0;
        
        for (i, c) in s.chars().enumerate() {
            if vowels.contains(&c) {
                let index = vowels.iter().position(|&x| x == c).unwrap() as u32;
                mask ^= 1 << index;
            }
            
            if let Some(&prev_index) = counts.get(&mask) {
                max_length = max_length.max(i as i32 - prev_index);
            } else {
                counts.insert(mask, i as i32);
            }
        }
        
        max_length

    }
}
