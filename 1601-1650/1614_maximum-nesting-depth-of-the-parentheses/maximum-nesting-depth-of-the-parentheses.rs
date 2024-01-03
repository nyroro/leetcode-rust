
impl Solution {
    pub fn max_depth(s: String) -> i32 {
        let mut stack = Vec::new();
        let mut max_depth = 0;
        let mut current_depth = 0;
        
        for c in s.chars() {
            if c == '(' {
                stack.push(c);
                current_depth += 1;
                max_depth = max_depth.max(current_depth);
            } else if c == ')' {
                stack.pop();
                current_depth -= 1;
            }
        }
        
        max_depth

    }
}
