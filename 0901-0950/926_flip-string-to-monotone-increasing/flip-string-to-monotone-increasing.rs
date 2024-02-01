
impl Solution {
    pub fn min_flips_mono_incr(s: String) -> i32 {
        let n = s.len();
        let s: Vec<char> = s.chars().collect();
        let mut flips = vec![0; n + 1];
        
        for i in 0..n {
            flips[i + 1] = flips[i] + (s[i] == '1') as i32;
        }
        
        let mut min_flips = n - flips[n];
        
        for i in 0..=n {
            let ones_left = flips[i];
            let zeros_right = n as i32 - i as i32 - (flips[n] - flips[i]);
            min_flips = min_flips.min(ones_left + zeros_right);
        }
        
        min_flips

    }
}
