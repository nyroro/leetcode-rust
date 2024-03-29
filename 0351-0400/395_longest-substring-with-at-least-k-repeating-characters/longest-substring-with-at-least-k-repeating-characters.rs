
use std::convert::TryInto;

impl Solution {
    pub fn longest_substring(s: String, k: i32) -> i32 {
        let mut max_len = 0;
        let n = s.len() as i32;
        
        if n == 0 || k > n {
            return 0;
        }
        
        let mut counter = [0; 26];
        for c in s.chars() {
            counter[c as usize - 'a' as usize] += 1;
        }
        
        for i in 0..n {
            if counter[s.chars().nth(i as usize).unwrap() as usize - 'a' as usize] < k {
                let left = Solution::longest_substring(s[..i as usize].to_string(), k);
                let right = Solution::longest_substring(s[(i+1) as usize..].to_string(), k);
                max_len = max_len.max(left).max(right);
                break;
            }
            
            if i == n - 1 {
                return n;
            }
        }
        
        max_len

    }
}
