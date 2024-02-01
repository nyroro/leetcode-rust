
impl Solution {
    pub fn longest_dup_substring(s: String) -> String {
        let n = s.len();
        let mut left = 1;
        let mut right = n;
        let mut result = String::new();
        
        while left <= right {
            let mid = left + (right - left) / 2;
            if let Some(substring) = Self::find_duplicate_substring(&s, mid) {
                result = substring;
                left = mid + 1;
            } else {
                right = mid - 1;
            }
        }
        
        result

    }
    
    fn find_duplicate_substring(s: &str, len: usize) -> Option<String> {
        let base = 26;
        let modulus = 2_u64.pow(32);
        let mut hash = 0_u64;
        let mut base_len = 1_u64;
        let mut seen = std::collections::HashMap::new();
        
        for i in 0..len {
            hash = (hash * base + (s.as_bytes()[i] - b'a') as u64) % modulus;
            base_len = (base_len * base) % modulus;
        }
        
        seen.insert(hash, 0);
        
        for start in 1..=(s.len() - len) {
            hash = (hash * base + (s.as_bytes()[start + len - 1] - b'a') as u64 - base_len * (s.as_bytes()[start - 1] - b'a') as u64) % modulus;
            if hash < 0 {
                hash += modulus;
            }
            if let Some(pos) = seen.get(&hash) {
                if &s[*pos..*pos + len] == &s[start..start + len] {
                    return Some(s[start..start + len].to_string());
                }
            }
            seen.insert(hash, start);
        }
        
        None

    }
}
