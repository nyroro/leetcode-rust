
impl Solution {
    pub fn reverse_str(s: String, k: i32) -> String {
        let mut chars: Vec<char> = s.chars().collect();
        let n = chars.len();
        let mut i = 0;

        while i < n {
            let end = (i + k as usize).min(n);
            chars[i..end].reverse();
            i += 2 * k as usize;
        }

        chars.into_iter().collect()
    }
}
