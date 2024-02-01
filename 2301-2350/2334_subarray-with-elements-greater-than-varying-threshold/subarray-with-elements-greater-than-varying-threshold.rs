
use std::collections::VecDeque;

impl Solution {
    pub fn valid_subarray_size(nums: Vec<i32>, threshold: i32) -> i32 {
        let mut nums = nums;
        nums.push(0);
        let mut stack: VecDeque<usize> = VecDeque::new();

        for (idx, &num) in nums.iter().enumerate() {
            while let Some(&top) = stack.back() {
                if num <= nums[top] {
                    let n = nums[stack.pop_back().unwrap()];
                    let k = if stack.is_empty() { idx } else { idx - stack.back().unwrap() - 1 };
                    if n > threshold / k as i32 {
                        return k as i32;
                    }
                } else {
                    break;
                }
            }
            stack.push_back(idx);
        }

        -1

    }
}
