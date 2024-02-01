
impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        let s = s.chars()
            .filter(|c| c.is_ascii_alphanumeric())
            .map(|c| c.to_ascii_lowercase())
            .collect::<Vec<char>>();
        
        if s.is_empty() {
            return true;
        }
        
        let (mut left, mut right) = (0, s.len() - 1);
        
        while left < right {
            if s[left] != s[right] {
                return false;
            }
            left += 1;
            right -= 1;
        }
        
        true

    }
}
