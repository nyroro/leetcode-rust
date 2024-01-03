
use std::collections::HashMap;



impl Solution {
    pub fn minimum_index(nums: Vec<i32>) -> i32 {
        let mut freq_map = HashMap::new();
        for &num in &nums {
            *freq_map.entry(num).or_insert(0) += 1;
        }
        
        let mut dominant_element = 0;
        for (&num, &freq) in &freq_map {
            if freq * 2 > nums.len() as i32 {
                dominant_element = num;
                break;
            }
        }
        
        let mut left_freq_map = HashMap::new();
        let mut right_freq_map = freq_map.clone();
        
        for i in 0..nums.len() - 1 {
            let num = nums[i];
            *left_freq_map.entry(num).or_insert(0) += 1;
            *right_freq_map.entry(num).or_insert(0) -= 1;
            
            if *left_freq_map.get(&dominant_element).unwrap_or(&0) * 2 > (i + 1) as i32 &&
               *right_freq_map.get(&dominant_element).unwrap_or(&0) * 2 > (nums.len() - i - 1) as i32 {
                return i as i32;
            }
        }
        
        -1

    }
}
