
impl Solution {
    pub fn longest_awesome(s: String) -> i32 {
        let mut mask = 0;
        let mut result = 0;
        let mut seen = vec![-1; 1024];
        seen[0] = 0;
        
        for (i, c) in s.chars().enumerate() {
            let digit = c.to_digit(10).unwrap() as usize;
            mask ^= 1 << digit;
            
            if seen[mask] != -1 {
                result = result.max(i as i32 + 1 - seen[mask]);
            } else {
                seen[mask] = i as i32 + 1;
            }
            
            for j in 0..10 {
                let flipped_mask = mask ^ (1 << j);
                if seen[flipped_mask] != -1 {
                    result = result.max(i as i32 + 1 - seen[flipped_mask]);
                }
            }
        }
        
        result

    }
}
