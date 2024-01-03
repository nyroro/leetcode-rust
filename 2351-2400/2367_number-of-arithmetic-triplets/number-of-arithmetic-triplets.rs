
use std::collections::HashMap;

impl Solution {
    pub fn arithmetic_triplets(nums: Vec<i32>, diff: i32) -> i32 {
        let mut count: HashMap<i32, i32> = HashMap::new();
        let mut result = 0;

        // 遍历数组，统计每个元素的出现次数

        for num in nums.iter() {
            *count.entry(*num).or_insert(0) += 1;
        }

        // 再次遍历数组，检查是否存在等差三元组

        for num in nums.iter() {
            let prev = num - diff;
            let next = num + diff;

            if count.contains_key(&prev) && count.contains_key(&next) {
                result += 1;
            }
        }

        result

    }
}
