
impl Solution {
    pub fn count_vowels(word: String) -> i64 {
        let vowels = ['a', 'e', 'i', 'o', 'u'];
        let mut result = 0;
        let word_chars: Vec<char> = word.chars().collect();
        
        for i in 0..word_chars.len() {
            for j in i..word_chars.len() {
                let mut vowel_count = 0;
                for k in i..=j {
                    if vowels.contains(&word_chars[k]) {
                        vowel_count += 1;
                    }
                }
                result += vowel_count;
            }
        }
        
        result as i64

    }
}
