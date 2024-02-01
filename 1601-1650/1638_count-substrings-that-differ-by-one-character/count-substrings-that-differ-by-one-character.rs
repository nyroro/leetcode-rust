
impl Solution {
    pub fn count_substrings(s: String, t: String) -> i32 {
        let mut count = 0;
        let s_chars: Vec<char> = s.chars().collect();
        let t_chars: Vec<char> = t.chars().collect();
        
        for i in 0..s.len() {
            for j in 0..t.len() {
                let mut diff_count = 0;
                for k in 0..s.len().min(t.len()) {
                    if s_chars[i + k] != t_chars[j + k] {
                        diff_count += 1;
                    }
                    if diff_count > 1 {
                        break;
                    }
                    if diff_count == 1 {
                        count += 1;
                    }
                }
            }
        }
        
        count

    }
}
