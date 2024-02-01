
impl Solution {
    pub fn common_chars(words: Vec<String>) -> Vec<String> {
        let mut char_count = std::collections::HashMap::new();
        
        // 统计第一个字符串中的字符出现次数

        for c in words[0].chars() {
            *char_count.entry(c).or_insert(0) += 1;
        }
        
        // 遍历剩余的字符串，更新字符出现次数

        for i in 1..words.len() {
            let mut word_count = std::collections::HashMap::new();
            for c in words[i].chars() {
                *word_count.entry(c).or_insert(0) += 1;
            }
            
            // 更新char_count中字符的出现次数

            for (c, count) in char_count.iter_mut() {
                if let Some(&word_count) = word_count.get(c) {
                    *count = count.min(word_count);
                } else {
                    *count = 0;
                }
            }
        }
        
        // 构造结果数组

        let mut result = Vec::new();
        for (c, count) in char_count.iter() {
            for _ in 0..*count {
                result.push(c.to_string());
            }
        }
        
        result

    }
}
