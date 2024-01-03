
use std::collections::HashMap;

impl Solution {
    pub fn intersection(nums: Vec<Vec<i32>>) -> Vec<i32> {
        let mut count = HashMap::new();
        
        for arr in &nums {
            for &num in arr {
                *count.entry(num).or_insert(0) += 1;
            }
        }
        
        let mut result = Vec::new();
        
        for (num, &freq) in &count {
            if freq == nums.len() {
                result.push(*num);
            }
        }
        
        result.sort();
        result

    }
}
