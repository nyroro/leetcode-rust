
impl Solution {
    pub fn count_prefixes(words: Vec<String>, s: String) -> i32 {
        // 初始化计数器

        let mut count = 0;
        
        // 遍历字符串数组

        for word in words {
            // 检查每个字符串是否是给定字符串的前缀

            if s.starts_with(&word) {
                // 如果是前缀，则增加计数器

                count += 1;
            }
        }
        
        // 返回前缀数量

        count

    }
}
