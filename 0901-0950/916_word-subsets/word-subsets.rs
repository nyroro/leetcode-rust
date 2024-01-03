
use std::collections::HashMap;

impl Solution {
    pub fn word_subsets(words1: Vec<String>, words2: Vec<String>) -> Vec<String> {
        let mut char_count = HashMap::new();
        
        for word in words2 {
            let mut temp_count = HashMap::new();
            for c in word.chars() {
                *temp_count.entry(c).or_insert(0) += 1;
            }
            for (c, count) in temp_count {
                *char_count.entry(c).or_insert(0) = char_count.get(&c).map_or(count, |&x| x.max(count));
            }
        }
        
        let mut result = Vec::new();
        
        for word in words1 {
            let mut temp_count = HashMap::new();
            for c in word.chars() {
                *temp_count.entry(c).or_insert(0) += 1;
            }
            let mut is_universal = true;
            for (c, count) in &char_count {
                if temp_count.get(&c).map_or(0, |&x| x) < *count {
                    is_universal = false;
                    break;
                }
            }
            if is_universal {
                result.push(word);
            }
        }
        
        result

    }
}
