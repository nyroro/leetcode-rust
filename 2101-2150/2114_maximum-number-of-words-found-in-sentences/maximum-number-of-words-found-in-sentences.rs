
impl Solution {
    pub fn most_words_found(sentences: Vec<String>) -> i32 {
        let mut max_word_count = 0;
        for sentence in sentences {
            let words: Vec<&str> = sentence.split(' ').collect();
            let word_count = words.len() as i32;
            if word_count > max_word_count {
                max_word_count = word_count;
            }
        }
        max_word_count

    }
}
