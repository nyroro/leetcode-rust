
impl Solution {
    pub fn make_good(s: String) -> String {
        let mut stack: Vec<char> = Vec::new();
        
        for c in s.chars() {
            if let Some(&top) = stack.last() {
                if top.to_ascii_lowercase() == c.to_ascii_lowercase() && top != c {
                    stack.pop();
                    continue;
                }
            }
            stack.push(c);
        }
        
        stack.into_iter().collect()
    }
}
