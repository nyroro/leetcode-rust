
use std::collections::HashMap;



impl Solution {
    pub fn count_bad_pairs(nums: Vec<i32>) -> i64 {
        let mut count_dict: HashMap<i32, i32> = HashMap::new();

        for (index, num) in nums.iter().enumerate() {
            let modified_value = *num - index as i32;
            *count_dict.entry(modified_value).or_insert(0) += 1;
        }

        let mut count: i64 = 0;
        for &value_count in count_dict.values() {
            count += i64::from(value_count) * (i64::from(value_count) - 1) / 2;
        }

        let nums_len = nums.len() as i64;
        let total_pairs = nums_len * (nums_len - 1) / 2;
        total_pairs - count

    }
}
