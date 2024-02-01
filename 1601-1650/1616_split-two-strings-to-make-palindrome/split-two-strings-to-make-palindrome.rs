
impl Solution {
    pub fn check_palindrome_formation(a: String, b: String) -> bool {
        // 辅助函数：检查字符串是否是回文的

        fn is_palindrome(s: &str) -> bool {
            let s = s.as_bytes();
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

        // 检查分割后的组合是否形成回文

        for i in 0..a.len() {
            if is_palindrome(&a[..i]) && is_palindrome(&b[i..]) {
                return true;
            }
            if is_palindrome(&b[..i]) && is_palindrome(&a[i..]) {
                return true;
            }
        }
        false

    }
}
