
impl Solution {
    pub fn consecutive_numbers_sum(n: i32) -> i32 {
        let mut count = 0;
        let mut start = 1;
        
        while start * (start - 1) < 2 * n {
            let numerator = n - (start * (start - 1) / 2);
            if numerator % start == 0 {
                count += 1;
            }
            start += 1;
        }
        
        count

    }
}
