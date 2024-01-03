
impl Solution {
    pub fn self_dividing_numbers(left: i32, right: i32) -> Vec<i32> {
        let mut result = Vec::new();
        
        for num in left..=right {
            let num_str = num.to_string();
            let mut is_self_dividing = true;
            
            for ch in num_str.chars() {
                let digit = ch.to_digit(10).unwrap() as i32;
                
                if digit == 0 || num % digit != 0 {
                    is_self_dividing = false;
                    break;
                }
            }
            
            if is_self_dividing {
                result.push(num);
            }
        }
        
        result

    }
}
