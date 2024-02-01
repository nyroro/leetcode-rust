
impl Solution {
    pub fn remove_stars(s: String) -> String {
        let mut result = String::new();
        let mut stack: Vec<usize> = Vec::new();
        
        for (i, c) in s.chars().enumerate() {
            if c == '*' {
                stack.push(i);
            } else if !stack.is_empty() {
                result.remove(stack.pop().unwrap());
            } else {
                result.push(c);
            }
        }
        
        result

    }
}
