
impl Solution {
    pub fn append_characters(s: String, t: String) -> i32 {
        let s_chars: Vec<char> = s.chars().collect();
        let t_chars: Vec<char> = t.chars().collect();
        let (mut i, mut j) = (0, 0);
        
        while i < s_chars.len() && j < t_chars.len() {
            if s_chars[i] == t_chars[j] {
                j += 1;
            }
            i += 1;
        }
        
        (t_chars.len() - j) as i32

    }
}
