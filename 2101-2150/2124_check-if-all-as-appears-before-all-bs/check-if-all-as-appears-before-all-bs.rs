
impl Solution {
    pub fn check_string(s: String) -> bool {
        let mut count_a = 0;
        let mut count_b = 0;
        
        for c in s.chars() {
            if c == 'a' {
                count_a += 1;
            } else if c == 'b' {
                if count_a > 0 {
                    return false;
                }
                count_b += 1;
            }
        }
        
        count_a >= count_b

    }
}
