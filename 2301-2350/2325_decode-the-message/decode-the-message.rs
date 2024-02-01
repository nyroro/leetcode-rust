
impl Solution {
    pub fn decode_message(key: String, message: String) -> String {
        // 创建替换表

        let mut substitution_table: Vec<char> = vec![' '; 26];
        
        // 构建替换表

        for c in key.chars() {
            if c != ' ' && substitution_table[(c as u8 - b'a') as usize] == ' ' {
                substitution_table[(c as u8 - b'a') as usize] = (substitution_table.len() as u8 + b'a') as char;
            }
        }
        
        // 解码消息

        let mut decoded_message = String::new();
        for c in message.chars() {
            if c == ' ' {
                decoded_message.push(' ');
            } else {
                let index = (c as u8 - b'a') as usize;
                decoded_message.push(substitution_table[index]);
            }
        }
        
        decoded_message

    }
}
