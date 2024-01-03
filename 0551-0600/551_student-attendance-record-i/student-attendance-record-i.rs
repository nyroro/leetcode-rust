
impl Solution {
    pub fn check_record(s: String) -> bool {
        let mut absent_count = 0;
        let mut late_count = 0;
        
        for c in s.chars() {
            if c == 'A' {
                absent_count += 1;
                late_count = 0;
            } else if c == 'L' {
                late_count += 1;
            } else {
                late_count = 0;
            }
            
            if absent_count >= 2 || late_count >= 3 {
                return false;
            }
        }
        
        true

    }
}
