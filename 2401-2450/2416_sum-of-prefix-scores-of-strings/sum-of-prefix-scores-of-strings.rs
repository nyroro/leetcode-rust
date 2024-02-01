
use std::collections::HashMap;

impl Solution {
    pub fn sum_prefix_scores(words: Vec<String>) -> Vec<i32> {
        let mut prefix_scores = vec![];
        let mut prefix_count = HashMap::new();

        for word in &words {
            let mut prefix = String::new();
            for ch in word.chars() {
                prefix.push(ch);
                *prefix_count.entry(prefix.clone()).or_insert(0) += 1;
            }
        }

        for word in &words {
            let mut score = 0;
            let mut prefix = String::new();
            for ch in word.chars() {
                prefix.push(ch);
                score += prefix_count[&prefix];
            }
            prefix_scores.push(score);
        }

        prefix_scores

    }
}
