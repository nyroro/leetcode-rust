
use std::collections::HashMap;

impl Solution {
    pub fn max_subarray_length(nums: Vec<i32>, k: i32) -> i32 {
        let mut ret = 0;
        let mut l = 0;
        let mut c = HashMap::new();

        for i in 0..nums.len() {
            *c.entry(nums[i]).or_insert(0) += 1;
            while l < i && *c.get(&nums[i]).unwrap() > k {
                if let Some(count) = c.get_mut(&nums[l]) {
                    *count -= 1;
                }
                l += 1;
            }
            ret = ret.max(i - l + 1);
        }
        ret as i32

    }
}
