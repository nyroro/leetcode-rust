
impl Solution {
    pub fn find_anagrams(s: String, p: String) -> Vec<i32> {
        let s_chars: Vec<char> = s.chars().collect();
        let p_chars: Vec<char> = p.chars().collect();
        let s_len = s_chars.len();
        let p_len = p_chars.len();
        let mut result: Vec<i32> = Vec::new();
        
        if s_len < p_len {
            return result;
        }
        
        let mut p_count: [i32; 26] = [0; 26];
        let mut s_count: [i32; 26] = [0; 26];
        
        for i in 0..p_len {
            p_count[p_chars[i] as usize - 'a' as usize] += 1;
            s_count[s_chars[i] as usize - 'a' as usize] += 1;
        }
        
        if p_count == s_count {
            result.push(0);
        }
        
        for i in p_len..s_len {
            s_count[s_chars[i] as usize - 'a' as usize] += 1;
            s_count[s_chars[i - p_len] as usize - 'a' as usize] -= 1;
            
            if p_count == s_count {
                result.push((i - p_len + 1) as i32);
            }
        }
        
        result

    }
}
