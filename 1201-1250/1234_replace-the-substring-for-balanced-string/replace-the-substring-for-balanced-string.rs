
impl Solution {
    pub fn balanced_string(s: String) -> i32 {
        let n = s.len();
        let mut counts = vec![0; 128];
        let mut left = 0;
        let mut result = n;
        
        for &c in s.as_bytes() {
            counts[c as usize] += 1;
        }
        
        for right in 0..n {
            counts[s.as_bytes()[right] as usize] -= 1;
            
            while left < n && counts['Q' as usize] <= n / 4

                && counts['W' as usize] <= n / 4

                && counts['E' as usize] <= n / 4

                && counts['R' as usize] <= n / 4 {
                result = result.min(right - left + 1);
                counts[s.as_bytes()[left] as usize] += 1;
                left += 1;
            }
        }
        
        result as i32

    }
}
