
impl Solution {
    pub fn count_consistent_strings(allowed: String, words: Vec<String>) -> i32 {
        let mut count = 0;
        for word in words {
            let mut is_consistent = true;
            for c in word.chars() {
                if !allowed.contains(c) {
                    is_consistent = false;
                    break;
                }
            }
            if is_consistent {
                count += 1;
            }
        }
        count

    }
}
