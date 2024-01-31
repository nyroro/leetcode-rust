
use std::collections::HashMap;

impl Solution {
    pub fn count_interesting_subarrays(nums: Vec<i32>, modulo: i32, k: i32) -> i64 {
        let new: Vec<i32> = nums.iter().map(|&val| if val % modulo == k { 1 } else { 0 }).collect();
        let mut d: HashMap<i32, i64> = HashMap::new();
        d.insert(0, 1);

        let (mut sm, mut ans) = (0, 0);

        for val in new {
            sm += val;
            ans += *d.get(&(sm - k).rem_euclid(modulo)).unwrap_or(&0);
            *d.entry(sm.rem_euclid(modulo)).or_insert(0) += 1;
        }

        ans

    }
}
