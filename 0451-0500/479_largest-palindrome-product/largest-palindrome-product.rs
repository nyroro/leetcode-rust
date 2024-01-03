
impl Solution {
    pub fn largest_palindrome(n: i32) -> i32 {
        if n == 1 {
            return 9;
        }
        
        let max = 10_u64.pow(n as u32) - 1;
        let min = 10_u64.pow(n as u32 - 1);
        
        for i in (min..=max).rev() {
            let palindrome = Self::construct_palindrome(i);
            for j in (min..=max).rev() {
                if j * j < palindrome {
                    break;
                }
                if palindrome % j == 0 && palindrome / j <= max {
                    return (palindrome % 1337) as i32;
                }
            }
        }
        
        0

    }
    
    fn construct_palindrome(num: u64) -> u64 {
        let s = num.to_string();
        let reversed = s.chars().rev().collect::<String>();
        let palindrome = format!("{}{}", s, reversed);
        palindrome.parse().unwrap()
    }
}
