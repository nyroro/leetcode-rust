
use std::collections::HashMap;

impl Solution {
    pub fn maximum_subarray_sum(nums: Vec<i32>, k: i32) -> i64 {
        let mut max_sum: i64 = 0;
        let mut window_sum: i64 = 0;
        let mut window_map: HashMap<i32, usize> = HashMap::new();
        let mut left: usize = 0;

        for right in 0..nums.len() {
            let right_num = nums[right];
            window_sum += right_num as i64;

            if (right as i32 - left as i32 + 1) > k {
                let left_num = nums[left];
                window_sum -= left_num as i64;
                left += 1;
            }

            if window_map.contains_key(&right_num) && window_map[&right_num] >= left {
                let index = window_map[&right_num];
                window_sum -= nums[index] as i64;
            }

            window_map.insert(right_num, right);

            if (right as i32 - left as i32 + 1) == k {
                max_sum = max_sum.max(window_sum);
            }
        }

        max_sum

    }
}
