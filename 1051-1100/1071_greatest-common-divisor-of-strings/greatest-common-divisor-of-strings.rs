
impl Solution {
    pub fn gcd_of_strings(str1: String, str2: String) -> String {
        if str1.len() < str2.len() {
            return Self::gcd_of_strings(str2, str1);
        }
        if str2.is_empty() {
            return str1;
        }
        if !str1.starts_with(&str2) {
            return String::new();
        }
        return Self::gcd_of_strings(str1[str2.len()..].to_string(), str2);
    }
}
