
impl Solution {
    pub fn backspace_compare(s: String, t: String) -> bool {
        let s_chars = Self::process_string(s);
        let t_chars = Self::process_string(t);
        
        if s_chars.len() != t_chars.len() {
            return false;
        }
        
        for i in 0..s_chars.len() {
            if s_chars[i] != t_chars[i] {
                return false;
            }
        }
        
        true

    }
    
    fn process_string(s: String) -> Vec<char> {
        let mut chars = Vec::new();
        
        for c in s.chars() {
            if c == '#' {
                chars.pop();
            } else {
                chars.push(c);
            }
        }
        
        chars

    }
}
