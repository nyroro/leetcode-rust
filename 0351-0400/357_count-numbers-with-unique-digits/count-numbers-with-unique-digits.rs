
impl Solution {
    pub fn count_numbers_with_unique_digits(n: i32) -> i32 {
        if n == 0 {
            return 1;
        }
        
        let mut count = 10;
        let mut unique_digits = 9;
        let mut available_digits = 9;
        
        for _ in 2..=n {
            unique_digits *= available_digits;
            count += unique_digits;
            available_digits -= 1;
        }
        
        count

    }
}
