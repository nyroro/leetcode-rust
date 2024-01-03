
impl Solution {
    pub fn rotated_digits(n: i32) -> i32 {
        let mut count = 0;
        for num in 1..=n {
            if Self::is_good_number(num) {
                count += 1;
            }
        }
        count

    }
    
    fn is_good_number(num: i32) -> bool {
        let mut num = num;
        let mut valid = false;
        while num > 0 {
            let digit = num % 10;
            if digit == 3 || digit == 4 || digit == 7 {
                return false;
            }
            if digit == 2 || digit == 5 || digit == 6 || digit == 9 {
                valid = true;
            }
            num /= 10;
        }
        valid

    }
}
