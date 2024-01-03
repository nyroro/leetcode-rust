
impl Solution {
    pub fn custom_sort_string(order: String, s: String) -> String {
        let mut counts = [0; 26]; // 创建一个长度为26的数组，用于记录每个字符在字符串s中出现的次数


        // 统计字符串s中每个字符出现的次数

        for ch in s.chars() {
            counts[ch as usize - 'a' as usize] += 1;
        }

        let mut result = String::new();

        // 按照order的顺序遍历字符

        for ch in order.chars() {
            // 将当前字符按照出现次数添加到结果字符串中

            for _ in 0..counts[ch as usize - 'a' as usize] {
                result.push(ch);
            }
            // 将当前字符的计数重置为0

            counts[ch as usize - 'a' as usize] = 0;
        }

        // 将剩余的字符按照字母顺序添加到结果字符串中

        for i in 0..26 {
            for _ in 0..counts[i] {
                result.push((i as u8 + 'a' as u8) as char);
            }
        }

        result

    }
}
