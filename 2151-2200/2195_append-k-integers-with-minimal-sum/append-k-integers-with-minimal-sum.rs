
use std::collections::HashSet;

impl Solution {
    pub fn minimal_k_sum(nums: Vec<i32>, k: i32) -> i64 {
        let mut unique_nums = HashSet::new();
        let mut ans: i64 = (k as i64) * ((k + 1) as i64) / 2;
        let mut level = k + 1;

        for &num in nums.iter() {
            unique_nums.insert(num);
        }

        let mut sorted_unique_nums: Vec<i32> = unique_nums.into_iter().collect();
        sorted_unique_nums.sort();

        for &t in sorted_unique_nums.iter() {
            if t < level {
                ans += (level - t) as i64;
                level += 1;
            }
        }

        ans

    }
}
