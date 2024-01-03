
impl Solution {
    pub fn reorder_spaces(text: String) -> String {
        let words: Vec<&str> = text.split_whitespace().collect();
        let word_count = words.len();
        let space_count = text.chars().filter(|&c| c == ' ').count();
        
        let mut result = String::new();
        
        if word_count == 1 {
            result.push_str(words[0]);
            result.push_str(&" ".repeat(space_count));
            return result;
        }
        
        let space_between = space_count / (word_count - 1);
        let extra_space = space_count % (word_count - 1);
        
        for i in 0..word_count {
            result.push_str(words[i]);
            
            if i < word_count - 1 {
                result.push_str(&" ".repeat(space_between));
            }
        }
        
        result.push_str(&" ".repeat(extra_space));
        
        result

    }
}
