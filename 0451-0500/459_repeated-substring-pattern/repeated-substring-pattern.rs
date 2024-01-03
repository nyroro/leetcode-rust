
impl Solution {
    pub fn repeated_substring_pattern(s: String) -> bool {
        let n = s.len();
        for i in 1..=n/2 {
            if n % i == 0 {
                let sub = &s[0..i];
                let new_str = sub.repeat(n/i);
                if new_str == s {
                    return true;
                }
            }
        }
        false

    }
}
