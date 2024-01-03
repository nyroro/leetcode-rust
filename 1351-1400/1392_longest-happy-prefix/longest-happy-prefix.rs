
impl Solution {
    pub fn longest_prefix(s: String) -> String {
        let s_bytes = s.as_bytes();
        let n = s_bytes.len();
        let mut lps = vec![0; n];
        let mut len = 0;
        let mut i = 1;
        
        while i < n {
            if s_bytes[i] == s_bytes[len] {
                len += 1;
                lps[i] = len;
                i += 1;
            } else {
                if len != 0 {
                    len = lps[len - 1];
                } else {
                    lps[i] = 0;
                    i += 1;
                }
            }
        }
        
        let prefix_len = lps[n - 1];
        if prefix_len > 0 {
            return String::from_utf8_lossy(&s_bytes[0..prefix_len]).to_string();
        } else {
            return String::new();
        }
    }
}
