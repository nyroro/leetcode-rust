
impl Solution {
    pub fn num_ways(s: String) -> i32 {
        let modulo = 1000000007;
        let ones_count = s.chars().filter(|&c| c == '1').count();
        
        if ones_count % 3 != 0 {
            return 0;
        }
        
        if ones_count == 0 {
            let n = s.len() as i64 - 1;
            return ((n * (n - 1) / 2) % modulo) as i32;
        }
        
        let ones_per_part = ones_count / 3;
        let mut count = 0;
        let mut ones = 0;
        let mut ways1 = 0;
        let mut ways2 = 0;
        
        if ones_per_part == 0 {
            return ((((s.len() - 1) as i64 * (s.len() - 2) as i64 / 2) % modulo) as i32);
        }
        
        for c in s.chars() {
            if c == '1' {
                ones += 1;
            }
            
            if ones == ones_per_part {
                count += 1;
            } else if ones == ones_per_part * 2 {
                ways2 += count;
            }
        }
        
        ((ways2 % modulo) as i32)
    }
}
