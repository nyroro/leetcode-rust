
use std::collections::HashMap;

impl Solution {
    pub fn max_operations(nums: Vec<i32>, k: i32) -> i32 {
        let mut counts = HashMap::new();
        let mut result = 0;

        // 计算每个数字的出现次数

        for &n in nums.iter() {
            *counts.entry(n).or_insert(0) += 1;
        }

        // 遍历数组并执行操作

        for &n in nums.iter() {
            if n == k - n {
                if *counts.get(&n).unwrap_or(&0) >= 2 {
                    result += 1;
                    *counts.get_mut(&n).unwrap() -= 2;
                }
            } else if *counts.get(&n).unwrap_or(&0) > 0 && *counts.get(&(k - n)).unwrap_or(&0) > 0 {
                result += 1;
                *counts.get_mut(&n).unwrap() -= 1;
                *counts.get_mut(&(k - n)).unwrap() -= 1;
            }
        }

        result

    }
}
