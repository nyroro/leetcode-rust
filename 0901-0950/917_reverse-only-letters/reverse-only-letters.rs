
impl Solution {
    pub fn reverse_only_letters(s: String) -> String {
        let mut chars: Vec<char> = s.chars().collect();
        let mut left = 0;
        let mut right = chars.len() - 1;
        
        while left < right {
            if !chars[left].is_ascii_alphabetic() {
                left += 1;
            } else if !chars[right].is_ascii_alphabetic() {
                right -= 1;
            } else {
                chars.swap(left, right);
                left += 1;
                right -= 1;
            }
        }
        
        chars.into_iter().collect()
    }
}
