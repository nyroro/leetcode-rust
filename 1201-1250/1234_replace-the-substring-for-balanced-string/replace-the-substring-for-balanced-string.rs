
use std::collections::HashMap;

impl Solution {
    pub fn balanced_string(s: String) -> i32 {
        let n = s.len();
        let mut counts = HashMap::new();
        let mut left = 0;
        let mut result = n;
        
        for right in 0..n {
            *counts.entry(s.chars().nth(right).unwrap()).or_insert(0) += 1;
            
            while left < n && counts.get(&'Q').unwrap_or(&0) > &(n / 4)
                && counts.get(&'W').unwrap_or(&0) > &(n / 4)
                && counts.get(&'E').unwrap_or(&0) > &(n / 4)
                && counts.get(&'R').unwrap_or(&0) > &(n / 4) {
                result = result.min(right - left + 1);
                *counts.get_mut(&s.chars().nth(left).unwrap()).unwrap() -= 1;
                left += 1;
            }
        }
        
        result as i32

    }
}
