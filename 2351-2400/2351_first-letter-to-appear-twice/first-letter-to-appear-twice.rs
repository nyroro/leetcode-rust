
impl Solution {
    pub fn repeated_character(s: String) -> char {
        let mut seen = std::collections::HashMap::new();
        for (i, c) in s.chars().enumerate() {
            if let Some(&prev) = seen.get(&c) {
                return prev;
            }
            seen.insert(c, c);
        }
        ' '
    }
}
