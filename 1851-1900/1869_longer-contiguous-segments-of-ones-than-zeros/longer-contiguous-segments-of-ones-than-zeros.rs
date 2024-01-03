
impl Solution {
    pub fn check_zero_ones(s: String) -> bool {
        let mut max_ones = 0;
        let mut max_zeros = 0;
        let mut current_ones = 0;
        let mut current_zeros = 0;
        
        for ch in s.chars() {
            match ch {
                '1' => {
                    current_ones += 1;
                    max_ones = max_ones.max(current_ones);
                    current_zeros = 0;
                }
                '0' => {
                    current_zeros += 1;
                    max_zeros = max_zeros.max(current_zeros);
                    current_ones = 0;
                }
                _ => {}
            }
        }
        
        max_ones > max_zeros

    }
}
