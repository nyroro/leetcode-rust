
impl Solution {
    pub fn detect_capital_use(word: String) -> bool {
        // Check if the word is entirely in uppercase

        if word.chars().all(|c| c.is_ascii_uppercase()) {
            return true;
        }
        
        // Check if the word is entirely in lowercase

        if word.chars().all(|c| c.is_ascii_lowercase()) {
            return true;
        }
        
        // Check if the first letter is uppercase and the rest of the letters are lowercase

        if let Some((first, rest)) = word.chars().next().map(|c| (c, &word[1..])) {
            if first.is_ascii_uppercase() && rest.chars().all(|c| c.is_ascii_lowercase()) {
                return true;
            }
        }
        
        // If none of the above conditions are met, return false

        false

    }
}
