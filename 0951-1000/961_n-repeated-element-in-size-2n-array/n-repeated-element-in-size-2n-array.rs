
use std::collections::HashSet;

impl Solution {
    pub fn repeated_n_times(nums: Vec<i32>) -> i32 {
        let mut set = HashSet::new();
        for num in nums {
            if set.contains(&num) {
                return num;
            }
            set.insert(num);
        }
        0 // 如果没有找到重复元素，返回0或其他合适的默认值

    }
}
