
impl Solution {
    pub fn count_valid_words(sentence: String) -> i32 {
        // 初始化有效单词数量为 0

        let mut valid_word_count = 0;
        
        // 将句子拆分成单词

        let words: Vec<&str> = sentence.split_whitespace().collect();
        
        // 遍历每个单词并验证

        for word in words {
            // 检查单词是否符合条件

            if Self::is_valid_word(word) {
                valid_word_count += 1;
            }
        }
        
        // 返回有效单词的数量

        valid_word_count

    }
    
    // 辅助函数，用于验证单词是否符合条件

    fn is_valid_word(word: &str) -> bool {
        // 检查是否只包含小写字母、连字符和标点符号

        if word.chars().all(|c| c.is_ascii_lowercase() || c == '-' || c.is_ascii_punctuation()) {
            // 检查连字符和标点符号的位置

            let mut hyphen_count = 0;
            let mut punctuation_count = 0;
            for (i, c) in word.chars().enumerate() {
                if c == '-' {
                    hyphen_count += 1;
                    if i == 0 || i == word.len() - 1 || !word.chars().nth(i - 1).unwrap().is_ascii_lowercase() || !word.chars().nth(i + 1).unwrap().is_ascii_lowercase() {
                        return false;
                    }
                } else if c.is_ascii_punctuation() {
                    punctuation_count += 1;
                    if i != word.len() - 1 {
                        return false;
                    }
                }
            }
            // 检查连字符和标点符号的数量

            hyphen_count <= 1 && punctuation_count <= 1

        } else {
            false

        }
    }
}
