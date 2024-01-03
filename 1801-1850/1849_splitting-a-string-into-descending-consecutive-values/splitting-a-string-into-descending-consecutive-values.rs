
impl Solution {
    pub fn split_string(s: String) -> bool {
        fn backtrack(s: &str, start: usize, prev: Option<u64>) -> bool {
            if start == s.len() {
                return true;
            }
            let mut num = 0;
            for i in start..s.len() {
                num = num * 10 + (s.as_bytes()[i] - b'0') as u64;
                if prev.map_or(false, |p| p - num == 1) && backtrack(s, i + 1, Some(num)) {
                    return true;
                }
                if prev.map_or(false, |p| p <= num) {
                    break;
                }
            }
            false

        }
        
        for i in 1..s.len() {
            let num = s[..i].parse::<u64>().unwrap();
            if backtrack(&s[i..], 0, Some(num)) {
                return true;
            }
        }
        false

    }
}
