
impl Solution {
    pub fn truncate_sentence(s: String, k: i32) -> String {
        let words: Vec<&str> = s.split_whitespace().collect(); // 将字符串按照空格分割成单词列表

        let truncated_words: Vec<&str> = words.iter().take(k as usize).cloned().collect(); // 取出前k个单词

        let truncated_sentence = truncated_words.join(" "); // 将单词连接成字符串

        truncated_sentence // 返回截断后的字符串

    }
}
