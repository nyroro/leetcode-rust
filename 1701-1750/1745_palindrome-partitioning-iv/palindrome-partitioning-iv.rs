
impl Solution {
    pub fn check_partitioning(s: String) -> bool {
        fn is_palindrome(s: &[u8]) -> bool {
            let (mut i, mut j) = (0, s.len() - 1);
            while i < j {
                if s[i] != s[j] {
                    return false;
                }
                i += 1;
                j -= 1;
            }
            true

        }
        
        let s = s.as_bytes();
        let n = s.len();
        
        for i in 1..n - 1 {
            for j in i + 1..n {
                if is_palindrome(&s[0..i]) && is_palindrome(&s[i..j]) && is_palindrome(&s[j..n]) {
                    return true;
                }
            }
        }
        
        false

    }
}
