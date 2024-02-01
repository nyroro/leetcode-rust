
impl Solution {
    pub fn digit_sum(s: String, k: i32) -> String {
        let mut s = s;
        while s.len() > k as usize {
            let mut next = String::new();
            let mut i = 0;
            while i < s.len() {
                if i + k as usize <= s.len() {
                    let sum: u32 = s[i..i + k as usize].chars().map(|c| c.to_digit(10).unwrap()).sum();
                    next.push_str(&sum.to_string());
                } else {
                    let sum: u32 = s[i..].chars().map(|c| c.to_digit(10).unwrap()).sum();
                    next.push_str(&sum.to_string());
                }
                i += k as usize;
            }
            s = next;
        }
        s

    }
}
