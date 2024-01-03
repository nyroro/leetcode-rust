
use std::collections::HashMap;

impl Solution {
    pub fn num_identical_pairs(nums: Vec<i32>) -> i32 {
        let mut count_map: HashMap<i32, i32> = HashMap::new();

        // 计算每个数字的出现次数

        for &num in &nums {
            *count_map.entry(num).or_insert(0) += 1;
        }

        let mut good_pairs = 0;

        // 计算每个数字的好对数并相加

        for &count in count_map.values() {
            good_pairs += (count * (count - 1)) / 2;
        }

        good_pairs

    }
}
