
impl Solution {
    pub fn is_long_pressed_name(name: String, typed: String) -> bool {
        let name_chars: Vec<char> = name.chars().collect();
        let typed_chars: Vec<char> = typed.chars().collect();
        let mut i = 0;
        let mut j = 0;
        
        while i < name_chars.len() && j < typed_chars.len() {
            if name_chars[i] == typed_chars[j] {
                i += 1;
                j += 1;
            } else if j > 0 && typed_chars[j] == typed_chars[j - 1] {
                j += 1;
            } else {
                return false;
            }
        }
        
        while j < typed_chars.len() && typed_chars[j] == typed_chars[j - 1] {
            j += 1;
        }
        
        i == name_chars.len() && j == typed_chars.len()
    }
}
