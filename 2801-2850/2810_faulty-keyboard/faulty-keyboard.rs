
impl Solution {
    pub fn final_string(s: String) -> String {
        let mut screen_text = String::new();
        
        for c in s.chars() {
            if c != 'i' {
                screen_text.push(c);
            } else {
                screen_text = screen_text.chars().rev().collect();
            }
        }
        
        screen_text

    }
}
