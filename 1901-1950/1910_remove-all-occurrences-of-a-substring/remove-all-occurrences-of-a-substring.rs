
impl Solution {
    pub fn remove_occurrences(s: String, part: String) -> String {
        let mut result = s.clone();
        while result.contains(&part) {
            if let Some(index) = result.find(&part) {
                result.replace_range(index..index + part.len(), "");
            }
        }
        result

    }
}
