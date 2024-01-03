
impl Solution {
    pub fn longest_word(words: Vec<String>) -> String {
        use std::collections::HashSet;

        let mut word_set: HashSet<String> = HashSet::new();
        for word in words {
            word_set.insert(word);
        }

        let mut longest_word = String::new();
        for word in &word_set {
            let mut is_valid = true;
            for i in 1..word.len() {
                if !word_set.contains(&word[..i]) {
                    is_valid = false;
                    break;
                }
            }
            if is_valid && (word.len() > longest_word.len() || (word.len() == longest_word.len() && word < &longest_word)) {
                longest_word = word.clone();
            }
        }

        longest_word

    }
}
