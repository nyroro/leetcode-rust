
use std::collections::HashMap;

impl Solution {
    pub fn num_special_equiv_groups(words: Vec<String>) -> i32 {
        let mut count = 0;
        let mut map: HashMap<Vec<u8>, i32> = HashMap::new();
        
        for word in words {
            let mut odd_chars = vec![0; 26];
            let mut even_chars = vec![0; 26];
            let word_bytes = word.as_bytes();
            
            for i in 0..word_bytes.len() {
                if i % 2 == 0 {
                    even_chars[(word_bytes[i] - b'a') as usize] += 1;
                } else {
                    odd_chars[(word_bytes[i] - b'a') as usize] += 1;
                }
            }
            
            let mut pattern = Vec::new();
            pattern.extend_from_slice(&even_chars);
            pattern.extend_from_slice(&odd_chars);
            
            *map.entry(pattern).or_insert(0) += 1;
        }
        
        for (_, &v) in map.iter() {
            if v > 0 {
                count += 1;
            }
        }
        
        count

    }
}
