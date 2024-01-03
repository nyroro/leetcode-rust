
impl Solution {
    pub fn is_acronym(words: Vec<String>, s: String) -> bool {
        // 创建一个空字符串用于存储连接后的首字母

        let mut acronym = String::new();
        
        // 遍历字符串数组 words

        for word in words {
            // 获取每个字符串的首字母并连接到 acronym 中

            acronym.push(word.chars().next().unwrap());
        }
        
        // 判断连接后的字符串是否与 s 相同

        acronym == s

    }
}
