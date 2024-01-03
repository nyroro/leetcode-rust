
impl Solution {
    pub fn is_prefix_string(s: String, words: Vec<String>) -> bool {
        let mut prefix = String::new();
        for word in words {
            prefix.push_str(&word);
            if prefix == s {
                return true;
            }
        }
        false

    }
}
