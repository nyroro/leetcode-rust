
impl Solution {
    pub fn is_strictly_palindromic(n: i32) -> bool {
        fn is_palindrome(mut num: i32, base: i32) -> bool {
            let mut digits = Vec::new();
            while num > 0 {
                digits.push((num % base) as u8);
                num /= base;
            }
            let mut left = 0;
            let mut right = digits.len() - 1;
            while left < right {
                if digits[left] != digits[right] {
                    return false;
                }
                left += 1;
                right -= 1;
            }
            true

        }

        for base in 2..=n-2 {
            if !is_palindrome(n, base) {
                return false;
            }
        }
        true

    }
}
