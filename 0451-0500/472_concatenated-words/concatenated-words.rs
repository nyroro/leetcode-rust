
impl Solution {
    pub fn find_all_concatenated_words_in_a_dict(words: Vec<String>) -> Vec<String> {
        let word_set: std::collections::HashSet<String> = words.into_iter().collect();
        let mut result: Vec<String> = Vec::new();
        
        for word in word_set.iter() {
            if Solution::is_concatenated_word(word, &word_set) {
                result.push(word.clone());
            }
        }
        
        result

    }
    
    fn is_concatenated_word(word: &str, word_set: &std::collections::HashSet<String>) -> bool {
        if word.is_empty() {
            return false;
        }
        
        let n = word.len();
        let mut dp: Vec<bool> = vec![false; n + 1];
        dp[0] = true;
        
        for i in 1..=n {
            for j in 0..i {
                if dp[j] && word_set.contains(&word[j..i]) {
                    dp[i] = true;
                    break;
                }
            }
        }
        
        dp[n]
    }
}
