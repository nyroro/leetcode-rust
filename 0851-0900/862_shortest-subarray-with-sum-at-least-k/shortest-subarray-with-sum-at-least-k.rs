
use std::collections::VecDeque;

impl Solution {
    pub fn shortest_subarray(nums: Vec<i32>, k: i32) -> i32 {
        let n = nums.len();
        let mut prefix_sums = vec![0i64; n + 1];
        let mut queue = VecDeque::new();
        let mut min_size = n + 1;

        for i in 0..n {
            prefix_sums[i + 1] = prefix_sums[i] as i64 + nums[i] as i64;
        }

        for (index, value) in prefix_sums.iter().enumerate() {
            while let Some(&last) = queue.back() {
                if *value <= prefix_sums[last] {
                    queue.pop_back();
                } else {
                    break;
                }
            }

            while let Some(first) = queue.front() {
                if *value - prefix_sums[*first] >= k as i64 {
                    min_size = min_size.min(index - queue.pop_front().unwrap());
                } else {
                    break;
                }
            }

            queue.push_back(index);
        }

        if min_size != n + 1 {
            min_size as i32

        } else {
            -1

        }
    }
}
