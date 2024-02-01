
impl Solution {
    pub fn minimum_length_encoding(words: Vec<String>) -> i32 {
        let mut set: std::collections::HashSet<String> = words.into_iter().collect();
        for word in &set {
            for i in 1..word.len() {
                set.remove(&word[i..]);
            }
        }
        let mut result = 0;
        for word in set {
            result += word.len() + 1;
        }
        result as i32

    }
}
