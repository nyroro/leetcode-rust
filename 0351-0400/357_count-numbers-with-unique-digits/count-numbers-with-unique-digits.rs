
impl Solution {
    pub fn count_numbers_with_unique_digits(n: i32) -> i32 {
        if n == 0 {
            return 1;
        }
        
        let mut count = 1;
        let mut factorial = 9;
        
        for i in 1..=n {
            count *= factorial;
            factorial -= 1;
        }
        
        count

    }
}
