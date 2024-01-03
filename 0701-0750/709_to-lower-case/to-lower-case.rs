
impl Solution {
    pub fn to_lower_case(s: String) -> String {
        let mut result = String::new();
        for c in s.chars() {
            if c.is_ascii_uppercase() {
                result.push(c.to_ascii_lowercase());
            } else {
                result.push(c);
            }
        }
        result

    }
}
