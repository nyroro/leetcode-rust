
impl Solution {
    pub fn number_of_lines(widths: Vec<i32>, s: String) -> Vec<i32> {
        let mut lines = 1;
        let mut width = 0;
        
        for c in s.chars() {
            let index = (c as u8 - b'a') as usize;
            let char_width = widths[index];
            
            if width + char_width > 100 {
                lines += 1;
                width = char_width;
            } else {
                width += char_width;
            }
        }
        
        vec![lines, width]
    }
}
