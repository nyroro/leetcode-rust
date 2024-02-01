
impl Solution {
    pub fn backspace_compare(s: String, t: String) -> bool {
        let s_stack = Self::build_stack(s);
        let t_stack = Self::build_stack(t);
        
        while let (Some(s_char), Some(t_char)) = (s_stack.pop(), t_stack.pop()) {
            if s_char != t_char {
                return false;
            }
        }
        
        s_stack.is_empty() && t_stack.is_empty()
    }
    
    fn build_stack(s: String) -> Vec<char> {
        let mut stack = Vec::new();
        
        for c in s.chars() {
            if c != '#' {
                stack.push(c);
            } else if !stack.is_empty() {
                stack.pop();
            }
        }
        
        stack

    }
}
