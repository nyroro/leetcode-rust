
impl Solution {
    pub fn can_transform(start: String, end: String) -> bool {
        let mut start_chars = start.chars().collect::<Vec<char>>();
        let mut end_chars = end.chars().collect::<Vec<char>>();
        
        let mut i = 0;
        let mut j = 0;
        while i < start_chars.len() && j < end_chars.len() {
            while i < start_chars.len() && start_chars[i] == 'X' {
                i += 1;
            }
            while j < end_chars.len() && end_chars[j] == 'X' {
                j += 1;
            }
            if (i < start_chars.len() && j == end_chars.len()) || (i == start_chars.len() && j < end_chars.len()) {
                return false;
            }
            if (i < start_chars.len() && j < end_chars.len()) && (start_chars[i] != end_chars[j] || (start_chars[i] == 'L' && i < j) || (start_chars[i] == 'R' && i > j)) {
                return false;
            }
            if (start_chars[i] == 'L' && i < j) || (start_chars[i] == 'R' && i > j) {
                return false;
            }
            if start_chars[i] == 'L' && start_chars[i] == end_chars[j] {
                i += 1;
                j += 1;
            } else if start_chars[i] == 'R' && start_chars[i] == end_chars[j] {
                i += 1;
                j += 1;
            } else {
                return false;
            }
        }
        
        while i < start_chars.len() && start_chars[i] == 'X' {
            i += 1;
        }
        while j < end_chars.len() && end_chars[j] == 'X' {
            j += 1;
        }
        
        i == start_chars.len() && j == end_chars.len()
    }
}
