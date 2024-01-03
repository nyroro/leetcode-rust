
use std::collections::HashMap;

impl Solution {
    pub fn maximum_number_of_string_pairs(words: Vec<String>) -> i32 {
        let mut pairs = 0;
        let mut word_map: HashMap<String, usize> = HashMap::new();

        for word in words {
            let reversed_word = word.chars().rev().collect::<String>();
            if let Some(val) = word_map.remove(&reversed_word) {
                pairs += 1;
            } else {
                word_map.insert(word, 1);
            }
        }

        pairs

    }
}
