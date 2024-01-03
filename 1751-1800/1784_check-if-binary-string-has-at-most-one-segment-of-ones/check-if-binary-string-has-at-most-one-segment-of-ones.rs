
impl Solution {
    pub fn check_ones_segment(s: String) -> bool {
        let mut ones_count = 0;
        let mut found_zero = false;
        
        for ch in s.chars() {
            if ch == '1' {
                if found_zero {
                    return false;
                }
                ones_count += 1;
            } else {
                found_zero = true;
            }
        }
        
        true

    }
}
