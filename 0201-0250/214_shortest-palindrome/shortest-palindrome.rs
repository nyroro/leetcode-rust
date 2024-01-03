
// 实现 shortest_palindrome 函数

impl Solution {
    pub fn shortest_palindrome(s: String) -> String {
        // 创建一个字符串的可变副本

        let mut rev = s.clone();
        // 将字符串反转

        rev = rev.chars().rev().collect();
        // 在反转后的字符串中找到最长的回文前缀

        for i in 0..s.len() {
            if s.starts_with(&rev[i..]) {
                // 如果找到回文前缀，则将剩余部分反转并添加到原字符串的前面

                return rev[0..i].to_string() + &s;
            }
        }
        // 如果没有找到回文前缀，则直接将整个反转后的字符串添加到原字符串的前面

        return rev.to_string() + &s;
    }
}
