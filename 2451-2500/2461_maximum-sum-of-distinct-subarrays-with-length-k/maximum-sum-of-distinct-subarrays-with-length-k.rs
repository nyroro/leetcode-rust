
use std::collections::HashSet;

impl Solution {
    pub fn maximum_subarray_sum(nums: Vec<i32>, k: i32) -> i64 {
        let mut max_sum: i64 = 0;
        let mut window_sum: i64 = 0;
        let mut window_set: HashSet<i32> = HashSet::new();
        let mut left: usize = 0;

        for right in 0..nums.len() {
            let right_num = nums[right];
            while window_set.contains(&right_num) {
                let left_num = nums[left];
                window_sum -= left_num as i64;
                window_set.remove(&left_num);
                left += 1;
            }
            window_sum += right_num as i64;
            window_set.insert(right_num);

            if (right as i32 - left as i32 + 1) == k {
                max_sum = max_sum.max(window_sum);
                let left_num = nums[left];
                window_sum -= left_num as i64;
                window_set.remove(&left_num);
                left += 1;
            }
        }

        max_sum

    }
}
