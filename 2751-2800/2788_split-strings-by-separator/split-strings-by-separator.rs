
impl Solution {
    pub fn split_words_by_separator(words: Vec<String>, separator: char) -> Vec<String> {
        let mut result = Vec::new();
        
        for word in words {
            let split_words: Vec<&str> = word.split(separator).collect();
            for split_word in split_words {
                if !split_word.is_empty() {
                    result.push(split_word.to_string());
                }
            }
        }
        
        result

    }
}
