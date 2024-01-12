
impl Solution {
    pub fn smallest_number(pattern: String) -> String {
        let n = pattern.len();
        let mut result = String::new();
        let mut stack = Vec::new();

        for i in 0..=n {
            stack.push(i + 1);
            if i == n || pattern.chars().nth(i).unwrap() == 'I' {
                while let Some(num) = stack.pop() {
                    result.push_str(&num.to_string());
                }
            }
        }

        result

    }
}
