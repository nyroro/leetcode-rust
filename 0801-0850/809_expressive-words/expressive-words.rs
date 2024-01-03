
impl Solution {
    pub fn expressive_words(s: String, words: Vec<String>) -> i32 {
        let mut count = 0;
        for word in words {
            if Self::is_stretchy(&s, &word) {
                count += 1;
            }
        }
        count

    }
    
    fn is_stretchy(s: &str, word: &str) -> bool {
        let s_chars: Vec<char> = s.chars().collect();
        let word_chars: Vec<char> = word.chars().collect();
        let mut i = 0;
        let mut j = 0;
        while i < s_chars.len() && j < word_chars.len() {
            if s_chars[i] != word_chars[j] {
                return false;
            }
            let mut s_count = 1;
            let mut word_count = 1;
            while i + 1 < s_chars.len() && s_chars[i] == s_chars[i + 1] {
                s_count += 1;
                i += 1;
            }
            while j + 1 < word_chars.len() && word_chars[j] == word_chars[j + 1] {
                word_count += 1;
                j += 1;
            }
            if s_count < word_count || (s_count < 3 && s_count != word_count) {
                return false;
            }
            i += 1;
            j += 1;
        }
        i == s_chars.len() && j == word_chars.len()
    }
}
