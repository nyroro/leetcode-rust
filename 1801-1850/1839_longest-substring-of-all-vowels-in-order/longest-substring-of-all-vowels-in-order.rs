
impl Solution {
    pub fn longest_beautiful_substring(word: String) -> i32 {
        let mut max_len = 0;
        let mut start = 0;
        let mut end = 0;
        let mut vowels = std::collections::HashSet::new();

        for (i, c) in word.chars().enumerate() {
            if vowels.is_empty() || c > *vowels.iter().max().unwrap() {
                vowels.insert(c);
            } else if c < *vowels.iter().max().unwrap() {
                start = i;
                vowels.clear();
            }
            end = i;

            if vowels.len() == 5 {
                max_len = max_len.max(end - start + 1);
            }
        }

        max_len as i32

    }
}
