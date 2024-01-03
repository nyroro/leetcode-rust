
impl Solution {
    pub fn remove_stars(s: String) -> String {
        let mut result = String::new();
        let mut stack: Vec<usize> = Vec::new();
        
        for (i, c) in s.chars().enumerate() {
            if c == '*' {
                if i > 0 && !stack.is_empty() {
                    result.pop();
                }
            } else {
                stack.push(i);
                result.push(c);
            }
        }
        
        result

    }
}
