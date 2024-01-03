
impl Solution {
    pub fn count_digits(num: i32) -> i32 {
        let mut tmp = num;
        let mut count = 0;
        
        while tmp != 0 {
            let rem = tmp % 10;
            if rem != 0 && num % rem == 0 {
                count += 1;
            }
            tmp /= 10;
        }
        
        count

    }
}
