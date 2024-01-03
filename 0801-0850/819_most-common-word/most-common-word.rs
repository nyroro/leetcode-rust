
use std::collections::HashMap;

impl Solution {
    pub fn most_common_word(paragraph: String, banned: Vec<String>) -> String {
        // 将段落中的标点符号替换为空格，并将段落转换为小写字母

        let cleaned_paragraph = paragraph

            .to_lowercase()
            .chars()
            .map(|c| if c.is_alphabetic() { c } else { ' ' })
            .collect::<String>();

        // 将被禁止的单词存储到HashSet中，以便快速判断一个单词是否被禁止

        let banned_set: std::collections::HashSet<String> = banned.into_iter().collect();

        // 统计每个单词的出现次数

        let mut word_count: HashMap<String, i32> = HashMap::new();
        for word in cleaned_paragraph.split_whitespace() {
            if !banned_set.contains(word) {
                *word_count.entry(word.to_string()).or_insert(0) += 1;
            }
        }

        // 找到出现次数最多的未被禁止的单词

        let mut max_count = 0;
        let mut most_common_word = String::new();
        for (word, count) in word_count {
            if count > max_count {
                max_count = count;
                most_common_word = word;
            }
        }

        most_common_word

    }
}
