
use std::collections::HashMap;

impl Solution {
    pub fn subarrays_with_k_distinct(nums: Vec<i32>, k: i32) -> i32 {
        let mut count = 0;
        let mut left1 = 0;
        let mut left2 = 0;
        let mut freq1 = HashMap::new();
        let mut freq2 = HashMap::new();
        
        for right in 0..nums.len() {
            *freq1.entry(nums[right]).or_insert(0) += 1;
            *freq2.entry(nums[right]).or_insert(0) += 1;
            
            while freq1.len() > k {
                *freq1.get_mut(&nums[left1]).unwrap() -= 1;
                if freq1[&nums[left1]] == 0 {
                    freq1.remove(&nums[left1]);
                }
                left1 += 1;
            }
            
            while freq2.len() >= k {
                *freq2.get_mut(&nums[left2]).unwrap() -= 1;
                if freq2[&nums[left2]] == 0 {
                    freq2.remove(&nums[left2]);
                }
                left2 += 1;
            }
            
            count += (left2 - left1) as i32;
        }
        
        count

    }
}
