
impl Solution {
    pub fn check_valid_string(s: String) -> bool {
        let mut stack = Vec::new();
        let mut star_stack = Vec::new();

        for (i, c) in s.chars().enumerate() {
            match c {
                '(' => stack.push(i),
                '*' => star_stack.push(i),
                ')' => {
                    if !stack.is_empty() {
                        stack.pop();
                    } else if !star_stack.is_empty() {
                        star_stack.pop();
                    } else {
                        return false;
                    }
                }
                _ => (),
            }
        }

        while !stack.is_empty() && !star_stack.is_empty() {
            if stack.pop().unwrap() > star_stack.pop().unwrap() {
                return false;
            }
        }

        stack.is_empty()
    }
}
