
use std::collections::HashMap;

impl Solution {
    pub fn max_rep_opt1(text: String) -> i32 {
        let mut char_count = HashMap::new();
        let mut max_length = 0;
        let mut max_same_char = 0;
        let mut same_char = 0;
        let mut last_char = ' ';
        
        for c in text.chars() {
            *char_count.entry(c).or_insert(0) += 1;
        }
        
        let text: Vec<char> = text.chars().collect();
        
        for i in 0..text.len() {
            if text[i] == last_char {
                same_char += 1;
            } else {
                same_char = 1;
            }
            
            max_same_char = max_same_char.max(same_char);
            
            if i >= 2 && text[i] == text[i - 2] {
                max_length = max_length.max(char_count[&text[i]] + 1);
            } else {
                max_length = max_length.max(max_same_char.min(char_count[&text[i]]));
            }
            
            last_char = text[i];
        }
        
        max_length as i32

    }
}
