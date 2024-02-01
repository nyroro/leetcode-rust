
impl Solution {
    pub fn count_good_substrings(s: String) -> i32 {
        let mut count = 0;
        let s = s.as_bytes();
        for i in 0..s.len() - 2 {
            if s[i] != s[i + 1] && s[i + 1] != s[i + 2] && s[i] != s[i + 2] {
                count += 1;
            }
        }
        count

    }
}
