
impl Solution {
    pub fn largest_palindrome(n: i32) -> i32 {
        let max = 10_i32.pow(n as u32) - 1;
        let min = 10_i32.pow(n as u32 - 1);
        let mut largest_palindrome = 0;
        
        for i in (min..=max).rev() {
            for j in (min..=max).rev() {
                let product = i * j;
                if product <= largest_palindrome {
                    break;
                }
                if Self::is_palindrome(product) {
                    largest_palindrome = product;
                    break;
                }
            }
        }
        
        largest_palindrome % 1337

    }
    
    fn is_palindrome(num: i32) -> bool {
        let s = num.to_string();
        let reversed = s.chars().rev().collect::<String>();
        s == reversed

    }
}
