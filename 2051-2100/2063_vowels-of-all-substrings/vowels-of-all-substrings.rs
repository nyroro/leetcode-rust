
impl Solution {
    pub fn count_vowels(word: String) -> i64 {
        let vowels = ['a', 'e', 'i', 'o', 'u'];
        let mut result = 0;
        let word_chars: Vec<char> = word.chars().collect();
        
        for i in 0..word_chars.len() {
            if vowels.contains(&word_chars[i]) {
                result += (i + 1) as i64 * (word_chars.len() - i) as i64;
            }
        }
        
        result

    }
}
