
impl Solution {
    pub fn convert_to_base7(num: i32) -> String {
        if num == 0 {
            return "0".to_string();
        }
        
        let mut result = String::new();
        let mut n = num.abs();
        
        while n > 0 {
            let digit = n % 7;
            result.insert(0, std::char::from_digit(digit as u32, 7).unwrap());
            n /= 7;
        }
        
        if num < 0 {
            result.insert(0, '-');
        }
        
        result

    }
}
