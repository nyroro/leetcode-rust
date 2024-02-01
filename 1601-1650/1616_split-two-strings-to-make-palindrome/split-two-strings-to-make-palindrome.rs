
impl Solution {
    pub fn check_palindrome_formation(a: String, b: String) -> bool {
        fn is_palindrome(s: &[u8]) -> bool {
            let (mut i, mut j) = (0, s.len() as i32 - 1);
            while i < j {
                if s[i as usize] != s[j as usize] {
                    return false;
                }
                i += 1;
                j -= 1;
            }
            true

        }

        fn check_palindrome(s1: &str, s2: &str) -> bool {
            let s1 = s1.as_bytes();
            let s2 = s2.as_bytes();
            let (mut i, mut j) = (0, s1.len() as i32 - 1);
            while i < j && s1[i as usize] == s2[j as usize] {
                i += 1;
                j -= 1;
            }
            is_palindrome(&s1[i as usize..=j as usize]) || is_palindrome(&s2[i as usize..=j as usize])
        }

        let a = a.as_bytes();
        let b = b.as_bytes();
        check_palindrome(std::str::from_utf8(a).unwrap(), std::str::from_utf8(b).unwrap()) || check_palindrome(std::str::from_utf8(b).unwrap(), std::str::from_utf8(a).unwrap())
    }
}
