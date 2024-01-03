
impl Solution {
    pub fn shortest_completing_word(license_plate: String, words: Vec<String>) -> String {
        // 创建一个 HashMap 用于存储 license_plate 中每个字母的频率

        let mut license_freq = std::collections::HashMap::new();
        for c in license_plate.chars() {
            if c.is_alphabetic() {
                let lowercase_c = c.to_ascii_lowercase();
                *license_freq.entry(lowercase_c).or_insert(0) += 1;
            }
        }
        
        let mut shortest_word = String::new();
        let mut shortest_length = std::usize::MAX;
        
        for word in words {
            // 创建一个 HashMap 用于存储当前单词中每个字母的频率

            let mut word_freq = std::collections::HashMap::new();
            for c in word.chars() {
                if c.is_alphabetic() {
                    let lowercase_c = c.to_ascii_lowercase();
                    *word_freq.entry(lowercase_c).or_insert(0) += 1;
                }
            }
            
            // 检查当前单词是否满足条件

            let mut is_completing_word = true;
            for (c, freq) in &license_freq {
                if *word_freq.get(c).unwrap_or(&0) < *freq {
                    is_completing_word = false;
                    break;
                }
            }
            
            // 更新最短单词和长度

            if is_completing_word && word.len() < shortest_length {
                shortest_word = word.clone();
                shortest_length = word.len();
            }
        }
        
        shortest_word

    }
}
