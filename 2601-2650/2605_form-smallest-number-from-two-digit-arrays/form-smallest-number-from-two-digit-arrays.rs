
use std::collections::HashSet;



impl Solution {
    pub fn min_number(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let set1: HashSet<_> = nums1.iter().cloned().collect();
        let set2: HashSet<_> = nums2.iter().cloned().collect();

        let mut result = i32::MAX;

        for &num1 in &nums1 {
            if set2.contains(&num1) {
                result = result.min(num1);
            }
        }

        for &num2 in &nums2 {
            if set1.contains(&num2) {
                result = result.min(num2);
            }
        }

        if result == i32::MAX {
            let min_num1 = *nums1.iter().min().unwrap();
            let min_num2 = *nums2.iter().min().unwrap();
            result = std::cmp::min(min_num1 * 10 + min_num2, min_num2 * 10 + min_num1);
        }

        result

    }
}
