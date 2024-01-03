
impl Solution {
    pub fn reformat_number(number: String) -> String {
        // 移除空格和破折号

        let cleaned_number: String = number.chars().filter(|c| c.is_digit(10)).collect();
        
        let mut result = String::new();
        let mut remaining_digits = cleaned_number.len();
        let mut i = 0;
        
        // 分组处理数字

        while remaining_digits > 4 {
            if i > 0 {
                result.push('-');
            }
            
            if remaining_digits >= 3 {
                result.push_str(&cleaned_number[i..i+3]);
                i += 3;
                remaining_digits -= 3;
            } else {
                result.push_str(&cleaned_number[i..]);
                break;
            }
        }
        
        // 处理剩余的数字

        if remaining_digits == 4 {
            if i > 0 {
                result.push('-');
            }
            result.push_str(&cleaned_number[i..i+2]);
            result.push('-');
            result.push_str(&cleaned_number[i+2..]);
        } else if remaining_digits == 3 {
            if i > 0 {
                result.push('-');
            }
            result.push_str(&cleaned_number[i..]);
        } else if remaining_digits == 2 {
            if i > 0 {
                result.push('-');
            }
            result.push_str(&cleaned_number[i..]);
        }
        
        result

    }
}
