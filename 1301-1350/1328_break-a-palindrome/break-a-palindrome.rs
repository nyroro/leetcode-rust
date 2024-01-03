
impl Solution {
    pub fn break_palindrome(palindrome: String) -> String {
        let mut chars: Vec<char> = palindrome.chars().collect();
        let len = chars.len();
        
        if len == 1 {
            return String::new();
        }
        
        for i in 0..len/2 {
            if chars[i] != 'a' {
                chars[i] = 'a';
                return chars.into_iter().collect();
            }
        }
        
        chars[len-1] = 'b';
        chars.into_iter().collect()
    }
}
