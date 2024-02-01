
impl Solution {
    pub fn minimum_length_encoding(words: Vec<String>) -> i32 {
        let mut set: std::collections::HashSet<String> = words.into_iter().collect();
        let mut remove_set: std::collections::HashSet<String> = std::collections::HashSet::new();
        for word in &set {
            for i in 1..word.len() {
                remove_set.insert(word[i..].to_string());
            }
        }
        set.retain(|word| !remove_set.contains(word));
        let mut result = 0;
        for word in set {
            result += word.len() + 1;
        }
        result as i32

    }
}
