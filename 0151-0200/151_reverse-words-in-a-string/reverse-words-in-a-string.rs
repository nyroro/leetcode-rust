
impl Solution {
    pub fn reverse_words(s: String) -> String {
        // 将字符串按照空格分割成单词数组

        let words: Vec<&str> = s.split_whitespace().collect();
        
        // 反转单词数组

        let reversed_words: Vec<&str> = words.into_iter().rev().collect();
        
        // 将反转后的单词数组用空格连接起来

        let result = reversed_words.join(" ");
        
        result

    }
}
