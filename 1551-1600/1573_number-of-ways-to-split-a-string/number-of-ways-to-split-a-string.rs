
impl Solution {
    pub fn num_ways(s: String) -> i32 {
        let modulo = 1000000007;
        let ones_count = s.chars().filter(|&c| c == '1').count();
        
        if ones_count % 3 != 0 {
            return 0;
        }
        
        if ones_count == 0 {
            return (((s.len() - 1) as i64 * (s.len() - 2) as i64 / 2) % modulo) as i32;
        }
        
        let ones_per_part = ones_count / 3;
        let mut count = 0;
        let mut ones = 0;
        
        for c in s.chars() {
            if c == '1' {
                ones += 1;
            }
            
            if ones == ones_per_part {
                count += 1;
            }
        }
        
        count

    }
}
