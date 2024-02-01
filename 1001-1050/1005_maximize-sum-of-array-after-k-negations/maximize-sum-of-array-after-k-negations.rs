
use std::collections::BinaryHeap;

impl Solution {
    pub fn largest_sum_after_k_negations(nums: Vec<i32>, k: i32) -> i32 {
        let mut nums = nums;
        nums.sort();

        let mut k = k;
        let mut i = 0;
        while k > 0 && i < nums.len() && nums[i] < 0 {
            nums[i] = -nums[i];
            k -= 1;
            i += 1;
        }

        if k > 0 && k % 2 != 0 {
            nums.sort();
            nums[0] = -nums[0];
        }

        nums.iter().sum()
    }
}
