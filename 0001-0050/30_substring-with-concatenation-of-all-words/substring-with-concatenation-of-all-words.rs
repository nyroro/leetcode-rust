
use std::collections::HashMap;

impl Solution {
    pub fn find_substring(s: String, words: Vec<String>) -> Vec<i32> {
        let mut result = Vec::new();
        let word_len = words[0].len();
        let total_len = words.len() * word_len;
        let mut word_map = HashMap::new();

        for word in &words {
            *word_map.entry(word.to_string()).or_insert(0) += 1;
        }

        for i in 0..word_len {
            let mut left = i;
            let mut right = i;
            let mut temp_map = word_map.clone();
            let mut count = 0;

            while right + word_len <= s.len() {
                let word = &s[right..right + word_len].to_string();
                right += word_len;

                if word_map.contains_key(word) {
                    *temp_map.get_mut(word).unwrap() -= 1;
                    if *temp_map.get(word).unwrap() >= 0 {
                        count += 1;
                    }
                }

                while count == words.len() {
                    if right - left == total_len {
                        result.push(left as i32);
                    }

                    let left_word = &s[left..left + word_len].to_string();
                    left += word_len;

                    if word_map.contains_key(left_word) {
                        if *temp_map.get_mut(left_word).unwrap() >= 0 {
                            count -= 1;
                        }
                        *temp_map.get_mut(left_word).unwrap() += 1;
                    }
                }
            }
        }

        result

    }
}
