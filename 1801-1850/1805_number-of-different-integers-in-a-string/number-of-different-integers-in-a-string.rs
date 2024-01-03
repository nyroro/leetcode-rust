
impl Solution {
    pub fn num_different_integers(word: String) -> i32 {
        let mut seen = std::collections::HashSet::new();
        let mut num = String::new();
        
        for c in word.chars() {
            if c.is_ascii_digit() {
                num.push(c);
            } else if !num.is_empty() {
                seen.insert(num.trim_start_matches('0').to_string());
                num.clear();
            }
        }
        
        if !num.is_empty() {
            seen.insert(num.trim_start_matches('0').to_string());
        }
        
        seen.len() as i32

    }
}
