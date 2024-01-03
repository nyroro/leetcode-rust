
use std::collections::HashSet;

impl Solution {
    pub fn has_all_codes(s: String, k: i32) -> bool {
        let mut codes = HashSet::new();
        let n = s.len();
        let k = k as usize;
        
        if n < k {
            return false;
        }
        
        for i in 0..=n-k {
            let code = &s[i..i+k];
            codes.insert(code);
        }
        
        codes.len() == 1 << k

    }
}
