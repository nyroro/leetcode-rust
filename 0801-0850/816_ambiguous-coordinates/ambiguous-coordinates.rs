
impl Solution {
    pub fn ambiguous_coordinates(s: String) -> Vec<String> {
        let s = &s[1..s.len()-1]; // 去除字符串两端的括号

        let mut result = Vec::new();
        
        for i in 1..s.len() {
            let x = &s[..i];
            let y = &s[i..];
            
            let x_coordinates = Self::get_coordinates(x);
            let y_coordinates = Self::get_coordinates(y);
            
            for x_coord in x_coordinates {
                for y_coord in &y_coordinates {
                    result.push(format!("({}, {})", x_coord, y_coord));
                }
            }
        }
        
        result

    }
    
    fn get_coordinates(s: &str) -> Vec<String> {
        let mut result = Vec::new();
        
        if s.len() == 1 || !s.starts_with('0') {
            result.push(s.to_string());
        }
        
        for i in 1..s.len() {
            let integer_part = &s[..i];
            let decimal_part = &s[i..];
            
            if (integer_part.len() == 1 || !integer_part.starts_with('0')) && !decimal_part.ends_with('0') {
                result.push(format!("{}.{}", integer_part, decimal_part));
            }
        }
        
        result

    }
}
