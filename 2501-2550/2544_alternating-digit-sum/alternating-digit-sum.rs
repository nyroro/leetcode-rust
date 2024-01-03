
impl Solution {
    pub fn alternate_digit_sum(n: i32) -> i32 {
        let mut sum = 0;
        let digits = n.to_string().chars().collect::<Vec<char>>();
        
        for i in 0..digits.len() {
            let digit = digits[i].to_digit(10).unwrap() as i32;
            let sign = if i % 2 == 0 { 1 } else { -1 };
            sum += sign * digit;
        }
        
        sum

    }
}
