
impl Solution {
    pub fn can_transform(start: String, end: String) -> bool {
        let start_chars: Vec<char> = start.chars().collect();
        let end_chars: Vec<char> = end.chars().collect();
        
        let mut i = 0;
        let mut j = 0;
        
        while i < start_chars.len() && j < end_chars.len() {
            if start_chars[i] == 'X' {
                i += 1;
                continue;
            }
            if end_chars[j] == 'X' {
                j += 1;
                continue;
            }
            if start_chars[i] != end_chars[j] {
                return false;
            }
            if (start_chars[i] == 'L' && i < j) || (start_chars[i] == 'R' && i > j) {
                return false;
            }
            if start_chars[i] == 'L' && end_chars[j] == 'R' {
                return false;
            }
            if start_chars[i] == 'R' && end_chars[j] == 'L' {
                return false;
            }
            i += 1;
            j += 1;
        }
        
        while i < start_chars.len() {
            if start_chars[i] != 'X' {
                return false;
            }
            i += 1;
        }
        while j < end_chars.len() {
            if end_chars[j] != 'X' {
                return false;
            }
            j += 1;
        }
        
        true

    }
}
