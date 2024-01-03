
impl Solution {
    pub fn find_lu_slength(strs: Vec<String>) -> i32 {
        let mut max_length = -1;
        for i in 0..strs.len() {
            let mut is_subsequence = false;
            for j in 0..strs.len() {
                if i != j && Solution::is_subsequence(&strs[i], &strs[j]) {
                    is_subsequence = true;
                    break;
                }
            }
            if !is_subsequence {
                max_length = max_length.max(strs[i].len() as i32);
            }
        }
        max_length

    }
    
    fn is_subsequence(s: &str, t: &str) -> bool {
        let mut i = 0;
        let mut j = 0;
        let s_chars: Vec<char> = s.chars().collect();
        let t_chars: Vec<char> = t.chars().collect();
        while i < s_chars.len() && j < t_chars.len() {
            if s_chars[i] == t_chars[j] {
                i += 1;
            }
            j += 1;
        }
        i == s_chars.len()
    }
}
