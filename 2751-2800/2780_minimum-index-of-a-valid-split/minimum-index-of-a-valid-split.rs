
use std::collections::HashMap;

impl Solution {
    pub fn minimum_index(nums: Vec<i32>) -> i32 {
        let mut freq_map = HashMap::new();
        for &num in &nums {
            *freq_map.entry(num).or_insert(0) += 1;
        }
        
        let mut left_freq_map = HashMap::new();
        let mut right_freq_map = freq_map.clone();
        
        for i in 0..nums.len() {
            let num = nums[i];
            *left_freq_map.entry(num).or_insert(0) += 1;
            *right_freq_map.entry(num).or_insert(0) -= 1;
            
            if left_freq_map.get(&num) == right_freq_map.get(&num) {
                return i as i32;
            }
        }
        
        -1

    }
}
