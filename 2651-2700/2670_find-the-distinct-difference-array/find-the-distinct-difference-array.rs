
use std::collections::HashSet;

impl Solution {
    pub fn distinct_difference_array(nums: Vec<i32>) -> Vec<i32> {
        let mut diff = Vec::new();

        for i in 0..nums.len() {
            let mut prefix = HashSet::new();
            let mut suffix = HashSet::new();
            let j = i + 1;

            for num in &nums[0..=i] {
                prefix.insert(num);
            }

            for num in &nums[j..] {
                suffix.insert(num);
            }

            diff.push((prefix.len() as i32) - (suffix.len() as i32));
        }

        diff

    }
}
