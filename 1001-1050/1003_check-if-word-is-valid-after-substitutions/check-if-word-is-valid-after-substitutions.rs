
impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut stack: Vec<char> = Vec::new();
        
        for c in s.chars() {
            if c == 'c' {
                if stack.pop() != Some('b') || stack.pop() != Some('a') {
                    return false;
                }
            } else {
                stack.push(c);
            }
        }
        
        stack.is_empty()
    }
}
