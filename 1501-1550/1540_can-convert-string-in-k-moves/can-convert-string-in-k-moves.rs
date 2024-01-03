
impl Solution {
    pub fn can_convert_string(s: String, t: String, k: i32) -> bool {
        let s_chars: Vec<char> = s.chars().collect();
        let t_chars: Vec<char> = t.chars().collect();
        
        if s_chars.len() != t_chars.len() {
            return false;
        }
        
        let mut operations: Vec<i32> = vec![0; 26];
        
        for i in 0..s_chars.len() {
            let c1 = s_chars[i] as i32;
            let c2 = t_chars[i] as i32;
            
            if c1 == c2 {
                continue;
            }
            
            let diff = c2 - c1;
            let mut count = diff;
            
            if diff < 0 {
                count += 26;
            }
            
            if count > k || count + operations[count as usize] * 26 > k {
                return false;
            }
            
            operations[count as usize] += 1;
        }
        
        true

    }
}
