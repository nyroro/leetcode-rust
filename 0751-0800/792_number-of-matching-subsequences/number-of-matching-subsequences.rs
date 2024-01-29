
impl Solution {
    pub fn num_matching_subseq(s: String, words: Vec<String>) -> i32 {
        fn is_subsequence(word: &str, s: &str) -> bool {
            let mut pos = -1;
            for char in word.chars() {
                pos = s[pos as usize + 1..].find(char).map_or(-1, |p| pos + 1 + p as i32);
                if pos == -1 {
                    return false;
                }
            }
            true

        }

        let mut count = 0;
        for word in words {
            if is_subsequence(&word, &s) {
                count += 1;
            }
        }
        count

    }
}
