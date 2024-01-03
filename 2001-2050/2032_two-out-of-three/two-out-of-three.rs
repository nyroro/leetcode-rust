
use std::collections::HashSet;

impl Solution {
    pub fn two_out_of_three(nums1: Vec<i32>, nums2: Vec<i32>, nums3: Vec<i32>) -> Vec<i32> {
        let mut result_set = HashSet::new();
        
        for num in &nums1 {
            if nums2.contains(num) || nums3.contains(num) {
                result_set.insert(*num);
            }
        }
        
        for num in &nums2 {
            if nums1.contains(num) || nums3.contains(num) {
                result_set.insert(*num);
            }
        }
        
        for num in &nums3 {
            if nums1.contains(num) || nums2.contains(num) {
                result_set.insert(*num);
            }
        }
        
        let result: Vec<i32> = result_set.into_iter().collect();
        result

    }
}
