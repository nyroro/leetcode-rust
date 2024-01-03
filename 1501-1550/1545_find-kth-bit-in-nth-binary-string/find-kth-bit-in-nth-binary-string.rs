
impl Solution {
    pub fn find_kth_bit(n: i32, k: i32) -> char {
        let s = Solution::generate_string(n, 1);
        s.chars().nth(k as usize - 1).unwrap()
    }
    
    fn generate_string(length: i32, level: i32) -> String {
        if level == length {
            return "0".to_string();
        }
        
        let prev_string = Solution::generate_string(length, level + 1);
        let reversed_inverted = prev_string.chars().rev().map(|c| if c == '0' { '1' } else { '0' }).collect::<String>();
        
        format!("{}1{}", prev_string, reversed_inverted)
    }
}
