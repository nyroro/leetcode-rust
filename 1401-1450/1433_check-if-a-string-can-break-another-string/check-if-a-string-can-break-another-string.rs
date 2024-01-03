
impl Solution {
    pub fn check_if_can_break(s1: String, s2: String) -> bool {
        let mut s1_chars: Vec<char> = s1.chars().collect();
        let mut s2_chars: Vec<char> = s2.chars().collect();
        
        s1_chars.sort();
        s2_chars.sort();
        
        let mut s1_breaks_s2 = true;
        let mut s2_breaks_s1 = true;
        
        for i in 0..s1_chars.len() {
            if s1_chars[i] < s2_chars[i] {
                s1_breaks_s2 = false;
            }
            if s2_chars[i] < s1_chars[i] {
                s2_breaks_s1 = false;
            }
        }
        
        s1_breaks_s2 || s2_breaks_s1

    }
}
