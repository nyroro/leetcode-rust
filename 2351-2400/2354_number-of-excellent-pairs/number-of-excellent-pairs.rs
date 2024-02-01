
use std::collections::HashSet;

impl Solution {
    pub fn count_excellent_pairs(nums: Vec<i32>, k: i32) -> i64 {
        let mut unique_nums = HashSet::new();
        for &num in nums.iter() {
            unique_nums.insert(num);
        }

        let mut bit_counts = vec![0; 32];
        for &num in unique_nums.iter() {
            let count = num.count_ones() as usize;
            bit_counts[count] += 1;
        }

        let mut result = 0;
        for i in 1..32 {
            for j in 1..32 {
                if i + j >= k {
                    result += bit_counts[i] * bit_counts[j];
                }
            }
        }

        result as i64

    }
}
