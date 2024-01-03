
use std::collections::HashMap;



impl Solution {
    pub fn min_operations(nums: Vec<i32>) -> i32 {
        let mut count = HashMap::new();
        for &num in &nums {
            *count.entry(num).or_insert(0) += 1;
        }
        let mut ans = 0;
        for &c in count.values() {
            if c < 2 {
                return -1;
            }
            ans += (c + 2) / 3;
        }
        ans

    }
}
