
impl Solution {
    pub fn interpret(command: String) -> String {
        // 创建一个新的字符串来存储解释后的结果

        let mut interpreted = String::new();
        
        // 遍历输入的字符串

        let mut chars = command.chars();
        while let Some(c) = chars.next() {
            // 根据题目规则进行解释

            if c == 'G' {
                interpreted.push('G');
            } else if c == '(' {
                if chars.next() == Some(')') {
                    interpreted.push('o');
                } else {
                    // 如果不是 "()"，则继续解释为 "al"
                    chars.next(); // 跳过 'a'
                    chars.next(); // 跳过 'l'
                    interpreted.push_str("al");
                }
            }
        }
        
        // 返回解释后的结果

        interpreted

    }
}
