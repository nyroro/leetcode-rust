
use std::collections::HashMap;

impl Solution {
    pub fn count_subarrays(nums: Vec<i32>, k: i32) -> i64 {
        let n = nums.len();
        let mut answer = 0;
        let high = *nums.iter().max().unwrap();
        let mut count = 0;
        let (mut left, mut right) = (0, 0);

        while right < n {
            if nums[right] == high {
                count += 1;
            }

            while count >= k {
                answer += (n - right) as i64;
                if nums[left] == high {
                    count -= 1;
                }
                left += 1;
            }

            right += 1;
        }

        answer

    }
}
