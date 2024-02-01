
impl Solution {
    pub fn count_homogenous(s: String) -> i32 {
        let modulo: i32 = 1_000_000_007;
        let mut count: i64 = 0;
        let mut current_count: i64 = 1;
        
        let chars: Vec<char> = s.chars().collect();
        
        for i in 1..chars.len() {
            if chars[i] == chars[i - 1] {
                current_count += 1;
            } else {
                count = (count + (current_count * (current_count + 1) / 2) % modulo) % modulo;
                current_count = 1;
            }
        }
        
        count = (count + (current_count * (current_count + 1) / 2) % modulo) % modulo;
        
        count as i32

    }
}
