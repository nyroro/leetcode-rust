
impl Solution {
    pub fn my_atoi(s: String) -> i32 {
        let mut result: i32 = 0;
        let mut sign: i32 = 1;
        let mut i: usize = 0;
        let chars: Vec<char> = s.chars().collect();
        
        // Ignore leading whitespace

        while i < chars.len() && chars[i] == ' ' {
            i += 1;
        }
        
        // Check for sign

        if i < chars.len() && (chars[i] == '-' || chars[i] == '+') {
            sign = if chars[i] == '-' { -1 } else { 1 };
            i += 1;
        }
        
        // Convert digits to integer

        while i < chars.len() && chars[i].is_digit(10) {
            let digit = chars[i].to_digit(10).unwrap() as i32;
            // Check for overflow

            if result > (i32::MAX - digit) / 10 {
                return if sign == 1 { i32::MAX } else { i32::MIN };
            }
            result = result * 10 + digit;
            i += 1;
        }
        
        result * sign

    }
}
