
impl Solution {
    pub fn remove_almost_equal_characters(word: String) -> i32 {
        let mut word_chars: Vec<char> = word.chars().collect();
        let mut count = 0;

        for i in 1..word_chars.len() {
            if word_chars[i] == word_chars[i - 1] || (word_chars[i] as i32 - word_chars[i - 1] as i32).abs() == 1 {
                word_chars[i] = '~';
                count += 1;
            }
        }

        count

    }
}
