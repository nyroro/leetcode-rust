
impl Solution {
    pub fn last_substring(s: String) -> String {
        let s = s.as_bytes();
        let n = s.len();
        let mut i = 0;
        let mut j = 1;
        let mut k = 0;
        while i + k < n && j + k < n {
            let si = s[i + k];
            let sj = s[j + k];
            if si == sj {
                k += 1;
            } else {
                if si > sj {
                    j += k + 1;
                } else {
                    i += k + 1;
                }
                if i == j {
                    j += 1;
                }
                k = 0;
            }
        }
        String::from_utf8_lossy(&s[i..]).to_string()
    }
}
