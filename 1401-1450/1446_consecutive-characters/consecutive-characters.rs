
impl Solution {
    pub fn max_power(s: String) -> i32 {
        let mut max_len = 0;
        let mut start = 0;
        let chars: Vec<char> = s.chars().collect();

        for end in 0..s.len() {
            if chars[end] != chars[start] {
                start = end;
            }
            let len = end - start + 1;
            max_len = max_len.max(len);
        }

        max_len as i32

    }
}
