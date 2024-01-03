
impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        if x < 0 {
            return false;
        }
        
        let mut rev = 0;
        let mut num = x;
        
        while num > 0 {
            let digit = num % 10;
            rev = rev * 10 + digit;
            num /= 10;
        }
        
        return rev == x;
    }
}
