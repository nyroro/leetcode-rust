
impl Solution {
    pub fn prime_palindrome(n: i32) -> i32 {
        let mut num = n;
        loop {
            if Self::is_palindrome(num) && Self::is_prime(num) {
                return num;
            }
            num += 1;
            if num > 10_000_000 && num < 100_000_000 {
                num = 100_000_000;
            }
        }
    }

    fn is_palindrome(num: i32) -> bool {
        let s = num.to_string();
        s == s.chars().rev().collect::<String>()
    }

    fn is_prime(num: i32) -> bool {
        if num < 2 {
            return false;
        }
        let mut i = 2;
        while i * i <= num {
            if num % i == 0 {
                return false;
            }
            i += 1;
        }
        true

    }
}
