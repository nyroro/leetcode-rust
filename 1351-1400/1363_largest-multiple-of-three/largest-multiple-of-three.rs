


impl Solution {
    pub fn largest_multiple_of_three(mut digits: Vec<i32>) -> String {
        digits.sort_unstable_by(|a, b| b.cmp(a));
        let sum: i32 = digits.iter().sum();
        let mut result = String::new();
        
        if sum % 3 == 1 {
            let mut removed = 0;
            for i in (0..digits.len()).rev() {
                if digits[i] % 3 == 1 {
                    digits.remove(i);
                    removed += 1;
                    break;
                }
            }
            if removed == 0 {
                let mut removed = 0;
                for i in (0..digits.len()).rev() {
                    if digits[i] % 3 == 2 {
                        digits.remove(i);
                        removed += 1;
                        if removed == 2 {
                            break;
                        }
                    }
                }
            }
        } else if sum % 3 == 2 {
            let mut removed = 0;
            for i in (0..digits.len()).rev() {
                if digits[i] % 3 == 2 {
                    digits.remove(i);
                    removed += 1;
                    break;
                }
            }
            if removed == 0 {
                let mut removed = 0;
                for i in (0..digits.len()).rev() {
                    if digits[i] % 3 == 1 {
                        digits.remove(i);
                        removed += 1;
                        if removed == 2 {
                            break;
                        }
                    }
                }
            }
        }
        
        if digits.is_empty() {
            return String::new();
        }
        
        for &digit in &digits {
            result.push_str(&digit.to_string());
        }
        
        if result.starts_with('0') {
            return "0".to_string();
        }
        
        result

    }
}
