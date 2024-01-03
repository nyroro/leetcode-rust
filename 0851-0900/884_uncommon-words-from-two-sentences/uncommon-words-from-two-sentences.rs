
impl Solution {
    pub fn uncommon_from_sentences(s1: String, s2: String) -> Vec<String> {
        use std::collections::HashMap;

        let mut word_count: HashMap<&str, i32> = HashMap::new();

        // 将 s1 和 s2 拼接成一个字符串

        let combined = format!("{} {}", s1, s2);

        // 统计每个单词的出现次数

        for word in combined.split_whitespace() {
            *word_count.entry(word).or_insert(0) += 1;
        }

        // 找出只出现一次的单词

        let uncommon_words: Vec<String> = word_count

            .into_iter()
            .filter(|(_, count)| *count == 1)
            .map(|(word, _)| word.to_string())
            .collect();

        uncommon_words

    }
}
