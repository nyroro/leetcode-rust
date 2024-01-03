
impl Solution {
    pub fn greatest_letter(s: String) -> String {
        let mut result = String::new();  // 创建一个空字符串来存储结果

        for c in s.chars() {
            if c.is_ascii_uppercase() && s.contains(c.to_ascii_lowercase()) {
                if result.is_empty() || c > result.chars().next().unwrap() {
                    result = c.to_string();  // 更新结果

                }
            }
        }
        result.to_ascii_uppercase()  // 返回找到的最大字母的大写形式

    }
}
