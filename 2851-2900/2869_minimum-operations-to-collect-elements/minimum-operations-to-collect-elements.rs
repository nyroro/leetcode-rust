
use std::collections::HashSet;

impl Solution {
    pub fn min_operations(nums: Vec<i32>, k: i32) -> i32 {
        let mut cnt = 0;
        let mut nums = nums;

        for i in (0..nums.len()).rev() {
            let j = (nums[i].abs() - 1) as usize;
            if j < k as usize && nums[j] > 0 {
                cnt += 1;
                nums[j] *= -1;
                if cnt == k {
                    return (nums.len() - i) as i32;
                }
            }
        }

        -1

    }
}
