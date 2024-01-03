
impl Solution {
    pub fn minimum_swap(s1: String, s2: String) -> i32 {
        let mut x_count = 0;
        let mut y_count = 0;
        let mut xy_count = 0;
        
        let s1_chars: Vec<char> = s1.chars().collect();
        let s2_chars: Vec<char> = s2.chars().collect();
        
        for i in 0..s1.len() {
            if s1_chars[i] == 'x' && s2_chars[i] == 'y' {
                x_count += 1;
            } else if s1_chars[i] == 'y' && s2_chars[i] == 'x' {
                y_count += 1;
            } else if s1_chars[i] == 'x' && s2_chars[i] == 'x' {
                xy_count += 1;
            }
        }
        
        if (x_count + y_count) % 2 != 0 {
            return -1;
        }
        
        (x_count / 2 + y_count / 2 + (x_count % 2) * 2) as i32

    }
}
