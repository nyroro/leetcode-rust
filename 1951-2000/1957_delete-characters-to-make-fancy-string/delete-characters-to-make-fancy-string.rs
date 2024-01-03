
impl Solution {
    pub fn make_fancy_string(s: String) -> String {
        // 创建一个空字符串来存储结果

        let mut ans = String::new();
        // 创建一个计数器来跟踪当前字符的连续出现次数

        let mut count = 0;
        
        // 遍历输入字符串的字符

        for c in s.chars() {
            // 如果结果字符串为空，或者当前字符不等于结果字符串的最后一个字符

            if ans.is_empty() || c != ans.chars().last().unwrap() {
                // 重置计数器

                count = 1;
            } else if count < 2 {
                // 如果当前字符等于结果字符串的最后一个字符且计数器小于2

                count += 1;
            } else {
                // 否则，跳过当前字符

                continue;
            }
            // 将当前字符添加到结果字符串

            ans.push(c);
        }
        
        // 返回结果字符串

        ans

    }
}
