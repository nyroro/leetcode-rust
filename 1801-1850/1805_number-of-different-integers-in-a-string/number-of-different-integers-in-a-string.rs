
impl Solution {
    pub fn num_different_integers(word: String) -> i32 {
        let mut cleaned_word = String::new();
        let mut seen = std::collections::HashSet::new();
        
        // Replace non-digit characters with space

        for c in word.chars() {
            if c.is_ascii_digit() {
                cleaned_word.push(c);
            } else {
                cleaned_word.push(' ');
            }
        }
        
        // Split the cleaned word by space and extract numbers

        for num in cleaned_word.split_whitespace() {
            let trimmed_num = num.trim_start_matches('0');
            if !trimmed_num.is_empty() {
                seen.insert(trimmed_num.to_string());
            }
        }
        
        seen.len() as i32

    }
}
