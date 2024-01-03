
impl Solution {
    pub fn minimum_length(s: String) -> i32 {
        let chars: Vec<char> = s.chars().collect();
        let mut left = 0;
        let mut right = chars.len() - 1;
        
        while left < right && chars[left] == chars[right] {
            let curr_char = chars[left];
            
            while left <= right && chars[left] == curr_char {
                left += 1;
            }
            
            while left <= right && chars[right] == curr_char {
                right -= 1;
            }
        }
        
        (right - left + 1) as i32

    }
}
