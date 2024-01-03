
impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut stack: Vec<char> = Vec::new();
        let chars: Vec<char> = s.chars().collect();
        
        for c in chars {
            match c {
                '(' | '[' | '{' => stack.push(c),
                ')' => {
                    if stack.is_empty() || stack.pop() != Some('(') {
                        return false;
                    }
                },
                ']' => {
                    if stack.is_empty() || stack.pop() != Some('[') {
                        return false;
                    }
                },
                '}' => {
                    if stack.is_empty() || stack.pop() != Some('{') {
                        return false;
                    }
                },
                _ => return false, // 非括号字符，直接返回false

            }
        }
        
        stack.is_empty()
    }
}
