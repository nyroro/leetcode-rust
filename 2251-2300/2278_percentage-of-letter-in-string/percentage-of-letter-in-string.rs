
impl Solution {
    pub fn percentage_letter(s: String, letter: char) -> i32 {
        let s_chars: Vec<char> = s.chars().collect();
        let mut count = 0;
        
        for c in s_chars {
            if c == letter {
                count += 1;
            }
        }
        
        (count * 100 / s_chars.len()) as i32

    }
}
