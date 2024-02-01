
impl Solution {
    pub fn replace_digits(s: String) -> String {
        let mut result = String::new();  // 步骤 1: 创建一个空字符串来存储结果

        for i in (1..s.len()).step_by(2) {  // 步骤 2: 遍历字符串的奇数索引位置

            let prev_char = s.chars().nth(i - 1).unwrap();  // 获取前一个索引位置上的字符

            let digit = s.chars().nth(i).unwrap().to_digit(10).unwrap();  // 获取当前索引位置上的数字

            let shifted_char = Self::shift(prev_char, digit);  // 调用 shift 函数得到替换后的字符

            result.push(prev_char);  // 将原始字符附加到结果字符串中

            result.push(shifted_char);  // 将替换后的字符附加到结果字符串中

        }
        if s.len() % 2 != 0 {  // 如果字符串的长度为奇数，末尾还有一个额外的字符

            result.push(s.chars().last().unwrap());  // 将最后一个字符附加到结果字符串中

        }
        result  // 步骤 4: 返回结果

    }
    
    fn shift(c: char, x: u32) -> char {
        let start = c as u8;
        let shifted = start + x as u8;
        shifted as char

    }
}
