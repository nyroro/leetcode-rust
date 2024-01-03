
impl Solution {
    pub fn remove_outer_parentheses(s: String) -> String {
        let mut result = String::new();
        let mut count = 0;

        for c in s.chars() {
            if c == '(' && count > 0 {
                result.push(c);
            }
            if c == ')' && count > 1 {
                result.push(c);
            }
            if c == '(' {
                count += 1;
            }
            if c == ')' {
                count -= 1;
            }
        }

        result

    }
}
