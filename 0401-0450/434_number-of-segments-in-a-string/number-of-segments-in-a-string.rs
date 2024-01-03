
impl Solution {
    pub fn count_segments(s: String) -> i32 {
        let mut count = 0;
        let chars: Vec<char> = s.chars().collect();
        
        for i in 0..chars.len() {
            if chars[i] != ' ' && (i == 0 || chars[i - 1] == ' ') {
                count += 1;
            }
        }
        
        count

    }
}
