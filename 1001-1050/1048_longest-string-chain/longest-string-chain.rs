
impl Solution {
    pub fn longest_str_chain(words: Vec<String>) -> i32 {
        let mut words = words;
        words.sort_by(|a, b| a.len().cmp(&b.len()));
        
        let mut word_map = std::collections::HashMap::new();
        let mut max_length = 1;
        
        for word in words {
            let mut max_chain_length = 1;
            
            for i in 0..word.len() {
                let predecessor = word[..i].to_string() + &word[i + 1..];
                let predecessor_length = *word_map.get(&predecessor).unwrap_or(&0);
                max_chain_length = max_chain_length.max(predecessor_length + 1);
            }
            
            word_map.insert(word, max_chain_length);
            max_length = max_length.max(max_chain_length);
        }
        
        max_length

    }
}
