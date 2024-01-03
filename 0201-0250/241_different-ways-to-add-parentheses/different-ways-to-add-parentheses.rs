
impl Solution {
    pub fn diff_ways_to_compute(expression: String) -> Vec<i32> {
        let mut result = Vec::new();
        
        for (i, c) in expression.chars().enumerate() {
            if c == '+' || c == '-' || c == '*' {
                let left = Self::diff_ways_to_compute(expression[..i].to_string());
                let right = Self::diff_ways_to_compute(expression[i+1..].to_string());
                
                for l in &left {
                    for r in &right {
                        match c {
                            '+' => result.push(l + r),
                            '-' => result.push(l - r),
                            '*' => result.push(l * r),
                            _ => (),
                        }
                    }
                }
            }
        }
        
        if result.is_empty() {
            result.push(expression.parse().unwrap());
        }
        
        result

    }
}
