
impl Solution {
    pub fn count_characters(words: Vec<String>, chars: String) -> i32 {
        use std::collections::HashMap;

        let mut char_counts: HashMap<char, i32> = HashMap::new();
        let mut result = 0;

        // 统计 chars 中每个字符的出现次数

        for ch in chars.chars() {
            *char_counts.entry(ch).or_insert(0) += 1;
        }

        // 遍历每个单词，检查是否为好字符串

        for word in words {
            let mut word_counts: HashMap<char, i32> = HashMap::new();
            let mut is_good = true;

            // 统计单词中每个字符的出现次数

            for ch in word.chars() {
                *word_counts.entry(ch).or_insert(0) += 1;
            }

            // 检查单词是否为好字符串

            for (ch, count) in word_counts {
                if !char_counts.contains_key(&ch) || count > *char_counts.get(&ch).unwrap() {
                    is_good = false;
                    break;
                }
            }

            // 如果单词是好字符串，则累加其长度

            if is_good {
                result += word.len() as i32;
            }
        }

        result

    }
}
