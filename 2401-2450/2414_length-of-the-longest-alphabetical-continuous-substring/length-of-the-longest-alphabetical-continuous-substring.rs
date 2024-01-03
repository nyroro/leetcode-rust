
impl Solution {
    pub fn longest_continuous_substring(s: String) -> i32 {
        let mut max_length = 1;
        let mut current_length = 1;
        
        let chars: Vec<char> = s.chars().collect();
        
        for i in 1..chars.len() {
            if chars[i] as u8 - chars[i - 1] as u8 == 1 {
                current_length += 1;
            } else {
                current_length = 1;
            }
            max_length = max_length.max(current_length);
        }
        
        max_length as i32

    }
}
