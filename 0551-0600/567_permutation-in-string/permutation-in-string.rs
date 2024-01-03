
impl Solution {
    pub fn check_inclusion(s1: String, s2: String) -> bool {
        let s1_chars: Vec<char> = s1.chars().collect();
        let s2_chars: Vec<char> = s2.chars().collect();
        let s1_len = s1_chars.len();
        let s2_len = s2_chars.len();
        
        if s1_len > s2_len {
            return false;
        }
        
        let mut s1_count: [i32; 26] = [0; 26];
        let mut s2_count: [i32; 26] = [0; 26];
        
        for i in 0..s1_len {
            s1_count[s1_chars[i] as usize - 'a' as usize] += 1;
            s2_count[s2_chars[i] as usize - 'a' as usize] += 1;
        }
        
        if s1_count == s2_count {
            return true;
        }
        
        for i in s1_len..s2_len {
            s2_count[s2_chars[i - s1_len] as usize - 'a' as usize] -= 1;
            s2_count[s2_chars[i] as usize - 'a' as usize] += 1;
            
            if s1_count == s2_count {
                return true;
            }
        }
        
        false

    }
}
