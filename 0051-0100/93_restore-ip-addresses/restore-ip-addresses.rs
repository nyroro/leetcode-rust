
impl Solution {
    pub fn restore_ip_addresses(s: String) -> Vec<String> {
        let mut result = Vec::new();
        let mut segments = Vec::new();
        Self::backtrack(&s, 0, &mut segments, &mut result);
        result

    }
    
    fn backtrack(s: &str, index: usize, segments: &mut Vec<String>, result: &mut Vec<String>) {
        if segments.len() == 4 && index == s.len() {
            result.push(segments.join("."));
            return;
        }
        
        if segments.len() < 4 && index < s.len() {
            for i in 1..=3 {
                if index + i > s.len() {
                    break;
                }
                
                let segment = &s[index..index+i];
                if Self::is_valid(segment) {
                    segments.push(segment.to_string());
                    Self::backtrack(s, index+i, segments, result);
                    segments.pop();
                }
            }
        }
    }
    
    fn is_valid(segment: &str) -> bool {
        if segment.len() > 1 && segment.starts_with('0') {
            return false;
        }
        
        if let Ok(num) = segment.parse::<u8>() {
            return num <= 255;
        }
        
        false

    }
}
