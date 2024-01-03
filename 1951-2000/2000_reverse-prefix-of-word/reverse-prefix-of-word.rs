
impl Solution {
    pub fn reverse_prefix(word: String, ch: char) -> String {
        // 使用 chars() 方法将字符串转换为字符迭代器

        let mut chars = word.chars();
        // 初始化一个空字符串用于存储结果

        let mut result = String::new();
        // 初始化一个变量用于记录字符 `ch` 的位置

        let mut index = 0;
        // 遍历字符迭代器

        while let Some(c) = chars.next() {
            // 将字符追加到结果字符串中

            result.push(c);
            // 如果当前字符等于 `ch`，则记录下位置并退出循环

            if c == ch {
                break;
            }
            // 更新位置

            index += 1;
        }
        // 如果找到了字符 `ch`，则将从开头到该位置的子串进行反转

        if let Some(pos) = word.find(ch) {
            let prefix = &word[..=pos];
            let reversed_prefix: String = prefix.chars().rev().collect();
            result = reversed_prefix + &word[pos + 1..];
        }
        // 返回结果字符串

        result

    }
}
