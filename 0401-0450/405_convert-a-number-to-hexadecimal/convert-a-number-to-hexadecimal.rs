
impl Solution {
    pub fn to_hex(num: i32) -> String {
        // 处理特殊情况，如果输入为0，则直接返回"0"
        if num == 0 {
            return "0".to_string();
        }
        
        // 定义一个字符数组，用于存储十六进制的字符

        let hex_chars: Vec<char> = vec!['0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e', 'f'];
        
        // 将输入的整数转换为无符号整数

        let mut unsigned_num = num as u32;
        
        // 定义一个空字符串，用于存储结果

        let mut result = String::new();
        
        // 循环处理每4位二进制数，转换为对应的十六进制字符

        while unsigned_num > 0 {
            // 提取最后4位二进制数

            let hex_digit = unsigned_num & 0xf;
            // 将对应的十六进制字符插入到结果字符串的开头

            result.insert(0, hex_chars[hex_digit as usize]);
            // 右移4位，继续处理下一个四位二进制数

            unsigned_num >>= 4;
        }
        
        // 返回结果字符串

        result

    }
}
