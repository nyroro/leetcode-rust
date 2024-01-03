
impl Solution {
    pub fn remove_palindrome_sub(s: String) -> i32 {
        if s.is_empty() {
            return 0;
        }
        
        if s.chars().rev().collect::<String>() == s {
            return 1;
        }
        
        return 2;
    }
}
