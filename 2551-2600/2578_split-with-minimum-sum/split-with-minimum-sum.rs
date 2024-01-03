
impl Solution {
    pub fn split_num(num: i32) -> i32 {
        let num_str = num.to_string();
        let mut count = vec![0; 10];
        
        for c in num_str.chars() {
            let digit = c.to_digit(10).unwrap() as usize;
            count[digit] += 1;
        }
        
        let mut num1 = 0;
        let mut num2 = 0;
        let mut is_first_digit = true;
        
        for i in 1..10 {
            while count[i] > 0 {
                if is_first_digit {
                    num1 = num1 * 10 + i as i32;
                } else {
                    num2 = num2 * 10 + i as i32;
                }
                count[i] -= 1;
                is_first_digit = !is_first_digit;
            }
        }
        
        num1 + num2

    }
}
